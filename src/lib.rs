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
//! #[derive(Debug, PartialEq, Identifiable, Queryable)]
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
//! # Upholding invariants when reading (`try_from`)
//!
//! By default the derive builds your newtype by wrapping the value read from
//! the database directly, which means an invalid value in the database becomes
//! an invalid instance of your type. If your newtype has invariants (for
//! example a private field that only accepts some values), add
//! `#[diesel_newtype(try_from = InnerType)]` or `#[diesel_newtype(try_from)]`
//! if the from type is identical to the type in the newtype. The read path
//! (`FromSql` and `Queryable`) will then deserialize into `InnerType` and call
//! `.try_into()` to build your newtype, so construction can fail:
//!
//! ```
//! # use std::convert::TryFrom;
//! # use diesel_derive_newtype::DieselNewType;
//! #[derive(Debug, PartialEq, Eq, Hash, DieselNewType)]
//! #[diesel_newtype(try_from = i32)]
//! pub struct Even(i32); // private inner field upholds an invariant; try_from applies it on DB reads
//!
//! #[derive(Debug)]
//! pub struct NotEvenError(i32);
//! # impl std::fmt::Display for NotEvenError {
//! #     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "not even") }
//! # }
//! impl std::error::Error for NotEvenError {}
//!
//! impl TryFrom<i32> for Even {
//!     type Error = NotEvenError;
//!     fn try_from(v: i32) -> Result<Self, NotEvenError> {
//!         if v % 2 == 0 { Ok(Even(v)) } else { Err(NotEvenError(v)) }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Notes:
//!
//! * The conversion accepts any type reachable via `.try_into()`, so an
//!   infallible `From<InnerType>` works too.
//! * The `TryFrom` error is cast into `Box<dyn std::error::Error + Send +
//!   Sync>`, it must implement `std::error::Error + Send + Sync + 'static`.
//! * Only the read path is affected. The write path (`ToSql`, `AsExpression`)
//!   always serializes the inner field directly.
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
//! #[diesel(table_name = my_entities)]
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
extern crate proc_macro2;

use proc_macro2::{Span, TokenStream};

#[proc_macro_derive(DieselNewType, attributes(diesel_newtype))]
#[doc(hidden)]
pub fn diesel_new_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse(input)
        .and_then(|ast| expand_sql_types(&ast))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_sql_types(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let wrapped_ty = validate_wrapped_type(ast)?;

    // `None` = wrap the inner value directly; `Some(ty)` = deserialize `ty` and
    // `.try_into()` the newtype. A bare `try_from` defaults `ty` to the field
    // type, which is the usual case.
    let from_ty = match parse_try_from(ast)? {
        TryFromAttr::None => None,
        TryFromAttr::Bare => Some(wrapped_ty.clone()),
        TryFromAttr::InnerType(ty) => Some(ty),
    };

    // Required to be able to insert/read from the db, don't allow searching.
    // The write path always serializes the inner field directly, so it is
    // unaffected by `try_from` (and needs no `Clone`/`Into`, unlike serde).
    let to_sql_impl = gen_tosql(name, wrapped_ty);
    let as_expr_impl = gen_asexpressions(name, wrapped_ty);

    // raw deserialization
    let from_sql_impl = gen_from_sql(name, wrapped_ty, from_ty.as_ref());

    // querying
    let queryable_impl = gen_queryable(name, wrapped_ty, from_ty.as_ref());

    // since our query doesn't take varargs it's fine for the DB to cache it
    let query_id_impl = gen_query_id(name);

    Ok(wrap_impls_in_const(&quote! {
        #to_sql_impl
        #as_expr_impl

        #from_sql_impl

        #queryable_impl

        #query_id_impl
    }))
}

/// Construct an error if the targetted type is not a newtype.
///
/// `#[derive(DieselNewType)]` only makes sense for a single-field tuple struct
/// so reject anything else with a pointed error rather than emitting code that
/// fails to compile downstream.
fn validate_wrapped_type(ast: &syn::DeriveInput) -> syn::Result<&syn::Type> {
    const HELP: &str = "#[derive(DieselNewType)] can only be used on a tuple \
        struct with exactly one field, e.g. `struct Foo(i64);`";
    let data = match &ast.data {
        syn::Data::Struct(data) => data,
        // enum / union: span the name so the error points at the type.
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            return Err(syn::Error::new_spanned(&ast.ident, HELP))
        }
    };
    match &data.fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(&fields.unnamed[0].ty),
        // unit struct has no fields to point at, so span the name instead.
        syn::Fields::Unit => Err(syn::Error::new_spanned(&ast.ident, HELP)),
        // wrong field count, or a named field: span the fields themselves.
        fields => Err(syn::Error::new_spanned(fields, HELP)),
    }
}

// Built and consumed exactly once per derive expansion, so the size gap between
// the unit variants and `InnerType`'s `syn::Type` is irrelevant here.
#[expect(clippy::large_enum_variant)]
enum TryFromAttr {
    /// no `try_from`: wrap the inner type with no constructor
    None,
    /// bare `#[diesel_newtype(try_from)]`: deserialize the newtype's own field type
    /// and `.try_into()` it (the common case, where the intermediate type *is*
    /// the wrapped type).
    Bare,
    /// `#[diesel_newtype(try_from = Ty)]`: deserialize `Ty` and `.try_into()` it.
    InnerType(syn::Type),
}

/// Parses `#[diesel_newtype(try_from = SomeType)]` or a bare `#[diesel_newtype(try_from)]`.
///
/// `.try_into()` upholds invariants on the read path; the infallible
/// `From` case is covered too, via the std blanket `Into` -> `TryInto` impls.
fn parse_try_from(ast: &syn::DeriveInput) -> syn::Result<TryFromAttr> {
    let mut try_from = TryFromAttr::None;
    for attr in &ast.attrs {
        if !attr.path().is_ident("diesel_newtype") {
            continue;
        }
        // Catch `#[diesel_newtype]` and `#[diesel_newtype()]` up front: syn's
        // own error for these ("unexpected end of input") is opaque, so point
        // the user at the real syntax instead.
        match &attr.meta {
            syn::Meta::List(list) if !list.tokens.is_empty() => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    attr,
                    "empty `#[diesel_newtype]` does nothing; write `#[diesel_newtype(try_from = Type)]`",
                ))
            }
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("try_from") {
                if !matches!(try_from, TryFromAttr::None) {
                    return Err(meta.error("duplicate `try_from` in #[diesel_newtype(...)]"));
                }
                // A bare `try_from` is terminated by `,` or the end of the
                // attribute; anything else must be `try_from = Type`, and
                // `meta.value()` reports a pointed "expected `=`" if the
                // separator is wrong (rather than a downstream "expected `,`").
                try_from = if meta.input.is_empty() || meta.input.peek(syn::Token![,]) {
                    TryFromAttr::Bare
                } else {
                    TryFromAttr::InnerType(meta.value()?.parse::<syn::Type>()?)
                };
                Ok(())
            } else {
                Err(meta.error(
                    "unsupported #[diesel_newtype(...)] key, expected `try_from` or `try_from = Type`",
                ))
            }
        })?;
    }
    Ok(try_from)
}

fn gen_tosql(name: &syn::Ident, wrapped_ty: &syn::Type) -> TokenStream {
    quote! {
        impl<ST, DB> diesel::serialize::ToSql<ST, DB> for #name
        where
            #wrapped_ty: diesel::serialize::ToSql<ST, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::sql_types::HasSqlType<ST>,
        {
            fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, DB>) -> diesel::serialize::Result
            {
                self.0.to_sql(out)
            }
        }
    }
}

fn gen_asexpressions(name: &syn::Ident, wrapped_ty: &syn::Type) -> TokenStream {
    quote! {

        impl<ST> diesel::expression::AsExpression<ST> for #name
        where
            diesel::internal::derives::as_expression::Bound<ST, #wrapped_ty>:
                diesel::expression::Expression<SqlType=ST>,
            ST: diesel::sql_types::SingleValue,
        {
            type Expression = diesel::internal::derives::as_expression::Bound<ST, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::internal::derives::as_expression::Bound::new(self)
            }
        }

        impl<'expr, ST> diesel::expression::AsExpression<ST> for &'expr #name
        where
            diesel::internal::derives::as_expression::Bound<ST, #wrapped_ty>:
                diesel::expression::Expression<SqlType=ST>,
            ST: diesel::sql_types::SingleValue,
        {
            type Expression = diesel::internal::derives::as_expression::Bound<ST, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::internal::derives::as_expression::Bound::new(self)
            }
        }

        impl<'expr2, 'expr, ST> diesel::expression::AsExpression<ST> for &'expr2 &'expr #name
        where
            diesel::internal::derives::as_expression::Bound<ST, #wrapped_ty>:
                diesel::expression::Expression<SqlType=ST>,
            ST: diesel::sql_types::SingleValue,
        {
            type Expression = diesel::internal::derives::as_expression::Bound<ST, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::internal::derives::as_expression::Bound::new(self)
            }
        }
    }
}

/// Builds the newtype from a value bound to `inner`: with `try_from`, a fallible
/// `try_into()` that preserves the conversion error (via `Into` into diesel's
/// boxed error); without it, a direct wrap. Shared by `FromSql` and `Queryable`
/// so the conversion semantics live in exactly one place.
fn gen_build_from_inner(name: &syn::Ident, try_from: Option<&syn::Type>) -> TokenStream {
    match try_from {
        Some(_) => quote! {
            ::std::convert::TryInto::try_into(inner)
                .map_err(::std::convert::Into::into)
        },
        None => quote! { ::std::result::Result::Ok(#name(inner)) },
    }
}

fn gen_from_sql(
    name: &syn::Ident,
    wrapped_ty: &syn::Type,
    try_from: Option<&syn::Type>,
) -> TokenStream {
    // When `try_from` is set, deserialize the intermediate type rather than the
    // inner field type; otherwise they're the same.
    let from_ty = try_from.unwrap_or(wrapped_ty);
    let build = gen_build_from_inner(name, try_from);
    quote! {
        impl<ST, DB> diesel::deserialize::FromSql<ST, DB> for #name
        where
            #from_ty: diesel::deserialize::FromSql<ST, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::sql_types::HasSqlType<ST>,
        {
            fn from_sql(raw: DB::RawValue<'_>) -> diesel::deserialize::Result<Self>
            {
                let inner: #from_ty =
                    diesel::deserialize::FromSql::<ST, DB>::from_sql(raw)?;
                #build
            }
        }
    }
}

fn gen_queryable(
    name: &syn::Ident,
    wrapped_ty: &syn::Type,
    try_from: Option<&syn::Type>,
) -> TokenStream {
    let from_ty = try_from.unwrap_or(wrapped_ty);
    let build = gen_build_from_inner(name, try_from);
    quote! {
        impl<ST, DB> diesel::deserialize::Queryable<ST, DB> for #name
        where
            #from_ty: diesel::deserialize::FromStaticSqlRow<ST, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::sql_types::HasSqlType<ST>,
        {
            type Row = #from_ty;

            fn build(inner: Self::Row) -> diesel::deserialize::Result<Self> {
                #build
            }
        }
    }
}

fn gen_query_id(name: &syn::Ident) -> TokenStream {
    quote! {
        impl diesel::query_builder::QueryId for #name {
            type QueryId = Self;
        }
    }
}

/// This guarantees that items we generate don't pollute the module scope
fn wrap_impls_in_const(item: &TokenStream) -> TokenStream {
    let dummy_const = syn::Ident::new("_", Span::call_site());
    quote! {
        const #dummy_const: () = {
            #item
        };
    }
}
