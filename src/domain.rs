use syn::{Ident, Ty};
use quote::Tokens;

use SqlDomainType;

pub(crate) fn gen_tosql(
    &SqlDomainType { domain_ty, diesel_ty }: &SqlDomainType,
    newtype: &Ident,
    wrapped_ty: &Ty
) -> Tokens {
    quote! {
        impl diesel::types::HasSqlType<#domain_ty> for diesel::sqlite::Sqlite
        {
            fn metadata(_: &()) -> diesel::sqlite::SqliteType {
                diesel::sqlite::SqliteType::#diesel_ty
            }
        }

        impl<DB: diesel::backend::Backend> diesel::types::ToSql<#domain_ty, DB> for #newtype
        where
            DB: diesel::types::HasSqlType<#domain_ty>,
            DB: diesel::types::HasSqlType<#wrapped_ty>,
            #wrapped_ty: diesel::types::ToSql<#wrapped_ty, DB>,
        {
            fn to_sql<W: ::std::io::Write>(&self, out: &mut diesel::types::ToSqlOutput<W, DB>)
            -> Result<diesel::types::IsNull, Box<::std::error::Error + Send + Sync>>
            {
                self.0.to_sql(out)
            }
        }
    }
}

pub(crate) fn gen_asexpresions(
    &SqlDomainType { domain_ty, diesel_ty }: &SqlDomainType,
    newtype: &Ident,
    wrapped_ty: &Ty,
) -> Tokens {
    quote! {
        impl diesel::expression::Expression for #domain_ty {
            type SqlType = diesel::types::#diesel_ty;
        }

        impl diesel::expression::AsExpression<#domain_ty> for #newtype
        where
            diesel::expression::bound::Bound<#domain_ty, #wrapped_ty>:
            diesel::expression::Expression<SqlType=#domain_ty>,
        {
            type Expression = diesel::expression::bound::Bound<#domain_ty, #wrapped_ty>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self.0)
            }
        }

        impl<'expr, #domain_ty> diesel::expression::AsExpression<#domain_ty> for &'expr #newtype
        where
            diesel::expression::bound::Bound<#domain_ty, #wrapped_ty>:
            diesel::expression::Expression<SqlType=#domain_ty>
        {
            type Expression = diesel::expression::bound::Bound<#domain_ty, &'expr #wrapped_ty>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(&self.0)
            }
        }
    }
}

pub(crate) fn gen_from_sql(
    &SqlDomainType { domain_ty, diesel_ty }: &SqlDomainType,
    newtype: &Ident,
    wrapped_ty: &Ty
) -> Tokens {
    quote! {
        impl<DB> diesel::types::FromSql<#domain_ty, DB> for #wrapped_ty
        where
            #wrapped_ty: diesel::types::FromSql<diesel::types::#diesel_ty, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<#domain_ty>,
        {
            fn from_sql(raw: Option<&<DB as diesel::backend::Backend>::RawValue>)
            -> Result<Self, Box<::std::error::Error + Send + Sync>>
            {
                diesel::types::FromSql::<diesel::types::#diesel_ty, DB>::from_sql(raw)
            }
        }

        impl<DB> diesel::types::FromSql<#domain_ty, DB> for #newtype
        where
            #wrapped_ty: diesel::types::FromSql<#domain_ty, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<#domain_ty>,
        {
            fn from_sql(raw: Option<&<DB as diesel::backend::Backend>::RawValue>)
            -> Result<Self, Box<::std::error::Error + Send + Sync>>
            {
                diesel::types::FromSql::<#domain_ty, DB>::from_sql(raw)
                    .map(#newtype)
            }
        }
    }
}

pub(crate) fn gen_from_sqlrow(
    ty: &SqlDomainType,
    name: &Ident,
    wrapped_ty: &Ty,
) -> Tokens {
    let domain_ty = ty.domain_ty;
    quote! {
        impl<DB> diesel::types::FromSqlRow<#domain_ty, DB> for #name
        where
            #wrapped_ty: diesel::types::FromSql<#domain_ty, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<#domain_ty>,
        {
            fn build_from_row<R: diesel::row::Row<DB>>(row: &mut R)
            -> Result<Self, Box<::std::error::Error + Send + Sync>>
            {
                diesel::types::FromSql::<#domain_ty, DB>::from_sql(row.take())
            }
        }
    }
}

pub(crate) fn gen_queryable(
    ty: &SqlDomainType,
    name: &Ident,
    wrapped_ty: &Ty,
) -> Tokens {
    let domain_ty = ty.domain_ty;
    quote! {
        impl<DB> diesel::query_source::Queryable<#domain_ty, DB> for #name
        where
            #wrapped_ty: diesel::types::FromSqlRow<#domain_ty, DB>,
            DB: diesel::backend::Backend,
            DB: diesel::types::HasSqlType<#domain_ty>,
        {
            type Row = #wrapped_ty;

            fn build(row: Self::Row) -> Self {
                #name(row)
            }
        }
    }
}

pub(crate) fn gen_query_fragment(
    &SqlDomainType { domain_ty, diesel_ty }: &SqlDomainType,
    newtype: &Ident,
    wrapped_ty: &Ty,
) -> Tokens {
    quote! {
        impl diesel::query_builder::Query for #domain_ty {
            type SqlType = #domain_ty;
        }

        impl<DB> diesel::query_builder::QueryFragment<DB> for #domain_ty
        where
            DB: diesel::backend::Backend,
            #wrapped_ty: diesel::query_builder::QueryFragment<DB>,
            diesel::types::#diesel_ty: diesel::query_builder::QueryFragment<DB>,
        {
            fn walk_ast(
                &self,
                mut out: diesel::query_builder::AstPass<DB>,
            ) -> QueryResult<()> {
                diesel::types::#diesel_ty.walk_ast(out)
            }
        }

        impl<DB> diesel::query_builder::QueryFragment<DB> for #newtype
        where
            DB: diesel::backend::Backend,
            #wrapped_ty: diesel::query_builder::QueryFragment<DB>,
            #domain_ty: diesel::query_builder::QueryFragment<DB>,
        {
            fn walk_ast(
                &self,
                mut out: diesel::query_builder::AstPass<DB>,
            ) -> QueryResult<()> {
                #domain_ty.walk_ast(out)
            }
        }
    }
}
