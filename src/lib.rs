#![recursion_limit = "1024"] // the new default in rust 1.19, quote! takes a lot

//! # `#[derive(DieselNewType)]`
//!
//! This crate exposes a single custom-derive macro `DieselNewType` which
//! implements `ToSql`, `FromSql`, `FromSqlRow`, `Queryable`, `AsExpression`
//! and `QueryId` for the single-field tuple struct ([NewType][]) it is applied
//! to.
//!
//! The goal of this project is that:
//!
//! * `derive(DieselNewType)` should be enough for you to use newtypes anywhere you
//!   would use their underlying types within Diesel. (plausibly successful)
//! * Should get the same compile-time guarantees when using your newtypes as
//!   expression elements in Diesel as you do in other rust code (depends on
//!   your desires, see [Limitations][], below.)
//!
//! [NewType]: https://aturon.github.io/features/types/newtype.html
//!
//! # Example
//!
//! This implementation:
//!
//! ```
//! #[macro_use]
//! extern crate diesel_derive_newtype;
//!
//! #[derive(DieselNewType)] // Doesn't need to be on its own line
//! #[derive(Debug, Hash, PartialEq, Eq)] // required by diesel
//! struct MyId(i64);
//! # fn main() {}
//! ```
//!
//! Allows you to use the `MyId` struct inside your entities as though they were
//! the underlying type:
//!
//! ```
//! # #[macro_use] extern crate diesel;
//! # #[macro_use] extern crate diesel_derive_newtype;
//! # use diesel::prelude::*;
//! table! {
//!     my_items {
//!         id -> Integer,
//!         val -> Integer,
//!     }
//! }
//!
//! # #[derive(DieselNewType)] // Doesn't need to be on its own line
//! # #[derive(Debug, Hash, PartialEq, Eq)] // required by diesel
//! # struct MyId(i64);
//! #[derive(Debug, PartialEq, Identifiable, Queryable, Associations)]
//! struct MyItem {
//!     id: MyId,
//!     val: u8,
//! }
//! # fn main() {}
//! ```
//!
//! Oooohhh. Ahhhh.
//!
//! See [the tests][] for a more complete example.
//!
//! [the tests]: https://github.com/quodlibetor/diesel-derive-newtype/blob/master/tests/db-roundtrips.rs
//!
//! # Limitations
//! [limitations]: #limitations
//!
//! The `DieselNewtype` derive does not create new _database_ types, or Diesel
//! serialization types. That is, if you have a `MyId(i64)`, this will use
//! Diesel's underlying `BigInt` type, which means that even though your
//! newtypes can be used anywhere the underlying type can be used, *the
//! underlying types, or any other newtypes of the same underlying type, can be
//! used as well*.
//!
//! At a certain point everything does become bits on the wire, so if we didn't
//! do it this way then Diesel would have to do it somewhere else, and this is
//! reasonable default behavior (it's pretty debuggable), but I'm investigating
//! auto-generating new proxy types as well to make it impossible to construct
//! an insert statement using a tuple or a mis-typed struct.
//!
//! Here's an example of that this type-hole looks like:
//!
//! ```rust,ignore
//! #[derive(Debug, Hash, PartialEq, Eq, DieselNewType)]
//! struct OneId(i64);
//!
//! #[derive(Debug, Hash, PartialEq, Eq, DieselNewType)]
//! struct OtherId(i64);
//!
//! #[derive(Debug, Clone, PartialEq, Identifiable, Insertable, Queryable)]
//! #[table_name="my_entities"]
//! pub struct MyEntity {
//!     id: OneId,
//!     val: i32,
//! }
//!
//! fn darn(conn: &Connection) {
//!     // shouldn't allow constructing the wrong type, but does
//!     let OtherId: Vec<OtherId> = my_entities
//!         .select(id)
//!         .filter(id.eq(OtherId(1)))  // shouldn't allow filtering by wrong type
//!         .execute(conn).unwrap();
//! }
//! ```
//!
//! See [`tests/should-not-compile.rs`](tests/should-not-compile.rs) for the
//! things I think should fail to compile.
//!
//! I believe that the root cause of this is that Diesel implements the various
//! expression methods for types that implement `AsExpression`, based on the
//! _SQL_ type, not caring about `self` and `other`'s Rust type matching. That
//! seems like a pretty good decision in general, but it is a bit unfortunate
//! here.
//!
//! I hope to find a solution that doesn't involve implementing every
//! `*Expression` trait manually with an extra bound, but for now you have to
//! keep in mind that the Diesel methods basically auto-transmute your data into
//! the underlying SQL type.


extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(DieselNewType)]
#[doc(hidden)]
pub fn diesel_new_type(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();

    expand_sql_types(&ast).parse().unwrap()
}

fn expand_sql_types(ast: &syn::DeriveInput) -> quote::Tokens {
    let body = match ast.body {
        syn::Body::Enum(_) => {
            panic!("#[derive(DieselNewType)] can only be used with structs with a single field")
        }
        syn::Body::Struct(ref data) if data.fields().len() != 1 => {
            panic!("#[derive(DieselNewType)] can only be used with structs with exactly one field")
        }
        syn::Body::Struct(ref data) => data.fields()[0].clone(),
    };

    let name = &ast.ident;
    let wrapped_ty = body.ty;

    // Required to be able to insert/read from the db, don't allow searching
    let to_sql_impl = gen_tosql(&name, &wrapped_ty);
    let as_expr_impl = gen_asexpresions(&name, &wrapped_ty);

    // raw deserialization
    let from_sql_impl = gen_from_sql(&name, &wrapped_ty);
    let from_sqlrow_impl = gen_from_sqlrow(&name, &wrapped_ty);

    // querying
    let queryable_impl = gen_queryable(&name, &wrapped_ty);

    // since our query doesn't take varargs it's fine for the DB to cache it
    let query_id_impl = gen_query_id(&name);

    wrap_impls_in_const(name, quote! {
        #to_sql_impl
        #as_expr_impl

        #from_sql_impl
        #from_sqlrow_impl

        #queryable_impl

        #query_id_impl
    })
}

fn gen_tosql(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {
        impl<ST, DB> diesel::types::ToSql<ST, DB> for #name
        where
            #wrapped_ty: diesel::types::ToSql<ST, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<ST>,
        {
            fn to_sql<W: ::std::io::Write>(&self, out: &mut diesel::types::ToSqlOutput<W, DB>)
            -> Result<diesel::types::IsNull, Box<::std::error::Error + Send + Sync>>
            {
                self.0.to_sql(out)
            }
        }
    }
}

fn gen_asexpresions(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {

        impl<ST> diesel::expression::AsExpression<ST> for #name
        where
            diesel::expression::bound::Bound<ST, #wrapped_ty>:
                diesel::expression::Expression<SqlType=ST>,
        {
            type Expression = diesel::expression::bound::Bound<ST, #wrapped_ty>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self.0)
            }
        }

        impl<'expr, ST> diesel::expression::AsExpression<ST> for &'expr #name
        where
            diesel::expression::bound::Bound<ST, #wrapped_ty>:
                diesel::expression::Expression<SqlType=ST>
        {
            type Expression = diesel::expression::bound::Bound<ST, &'expr #wrapped_ty>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(&self.0)
            }
        }
    }
}

fn gen_from_sql(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {
        impl<ST, DB> diesel::types::FromSql<ST, DB> for #name
        where
            #wrapped_ty: diesel::types::FromSql<ST, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<ST>,
        {
            fn from_sql(raw: Option<&<DB as diesel::backend::Backend>::RawValue>)
            -> Result<Self, Box<::std::error::Error + Send + Sync>>
            {
                diesel::types::FromSql::<ST, DB>::from_sql(raw)
                    .map(#name)
            }
        }
    }
}

fn gen_from_sqlrow(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {
        impl<ST, DB> diesel::types::FromSqlRow<ST, DB> for #name
        where
            #wrapped_ty: diesel::types::FromSql<ST, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<ST>,
        {
            fn build_from_row<R: diesel::row::Row<DB>>(row: &mut R)
            -> Result<Self, Box<::std::error::Error + Send + Sync>>
            {
                diesel::types::FromSql::<ST, DB>::from_sql(row.take())
            }
        }
    }
}

fn gen_queryable(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {
        impl<ST, DB> diesel::query_source::Queryable<ST, DB> for #name
        where
            #wrapped_ty: diesel::types::FromSqlRow<ST, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<ST>,
        {
            type Row = #wrapped_ty;

            fn build(row: Self::Row) -> Self {
                #name(row)
            }
        }
    }
}

fn gen_query_id(name: &syn::Ident) -> quote::Tokens {
    quote! {
        impl diesel::query_builder::QueryId for #name {
            type QueryId = Self;
        }
    }
}

/// This guarantees that items we generate don't polute the module scope
///
/// We use the const name as a form of documentation of the generated code
fn wrap_impls_in_const(ty_name: &syn::Ident, item: quote::Tokens) -> quote::Tokens {
    let name = ty_name.to_string().to_uppercase();
    let dummy_const: syn::Ident = format!("_IMPL_DIESEL_NEW_TYPE_FOR_{}", name).into();
    quote! {
        const #dummy_const: () = {
            extern crate diesel;
            #item
        };
    }
}
