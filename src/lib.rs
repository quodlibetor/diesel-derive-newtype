#![recursion_limit="1024"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

extern crate diesel;

use proc_macro::TokenStream;

#[proc_macro_derive(DieselNewType)]
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

    let to_sql_impl = gen_tosql(&name, &wrapped_ty);
    let from_sql_impl = gen_from_sql(&name, &wrapped_ty);
    let from_sqlrow_impl = gen_from_sqlrow(&name, &wrapped_ty);
    let query_id_impl = gen_query_id(&name);

    quote! {
        #to_sql_impl
        #from_sql_impl
        #from_sqlrow_impl
        #query_id_impl
    }
}

fn gen_tosql(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {
        impl<ST, DB> ::diesel::types::ToSql<ST, DB> for #name
        where
            #wrapped_ty: ::diesel::types::ToSql<ST, DB>,
            DB: ::diesel::backend::Backend,
            DB: ::diesel::types::HasSqlType<ST>,
        {
            fn to_sql<W: ::std::io::Write>(&self, out: &mut W)
            -> Result<::diesel::types::IsNull, Box<::std::error::Error + Send + Sync>>
            {
                self.0.to_sql(out)
            }
        }
    }
}

fn gen_from_sql(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {
        impl<ST, DB> ::diesel::types::FromSql<ST, DB> for #name
        where
            #wrapped_ty: ::diesel::types::FromSql<ST, DB>,
            DB: ::diesel::backend::Backend,
            DB: ::diesel::types::HasSqlType<ST>,
        {
            fn from_sql(raw: Option<&<DB as ::diesel::backend::Backend>::RawValue>)
            -> Result<Self, Box<::std::error::Error + Send + Sync>>
            {
                ::diesel::types::FromSql::<ST, DB>::from_sql(raw)
                    .map(#name)
            }
        }
    }
}

fn gen_from_sqlrow(name: &syn::Ident, wrapped_ty: &syn::Ty) -> quote::Tokens {
    quote! {
        impl<ST, DB> ::diesel::types::FromSqlRow<ST, DB> for #name
        where
            #wrapped_ty: ::diesel::types::FromSql<ST, DB>,
            DB: ::diesel::backend::Backend,
            DB: ::diesel::types::HasSqlType<ST>,
        {
            fn build_from_row<R: ::diesel::row::Row<DB>>(row: &mut R)
            -> Result<Self, Box<::std::error::Error + Send + Sync>>
            {
                ::diesel::types::FromSql::<ST, DB>::from_sql(row.take())
            }
        }

    }
}

fn gen_query_id(name: &syn::Ident) -> quote::Tokens {
    quote! {
        impl ::diesel::query_builder::QueryId for #name {
            type QueryId = Self;
            fn has_static_query_id() -> bool {
                true
            }
        }
    }
}
