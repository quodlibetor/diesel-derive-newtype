// This is just domain-type.rs run through cargo-expand, with some features and
// uses added so that it (almost) compiles

#![feature(prelude_import, structural_match, rustc_attrs, fmt_internals, rt, print_internals, libstd_sys_internals, test, derive_eq, custom_attribute)]
#![no_std]
#![allow(unused_macros)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel_newtype;

use std::{io, rt};

use diesel::prelude::*;
use diesel::expression::sql;
use diesel::sqlite::SqliteConnection;
use diesel::{insertable, backend, expression, query_builder, associations, query_source, types, result};

struct IdType;

//#[sql_type(IdType, Integer)]
#[structural_match]
pub struct MyId(i32);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for MyId {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MyId(ref __self_0_0) => {
                let mut builder = __arg_0.debug_tuple("MyId");
                let _ = builder.field(&&(*__self_0_0));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for MyId {
    #[inline]
    fn clone(&self) -> MyId {
        match *self {
            MyId(ref __self_0_0) => MyId(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for MyId {
    #[inline]
    fn eq(&self, __arg_0: &MyId) -> bool {
        match *__arg_0 {
            MyId(ref __self_1_0) => match *self {
                MyId(ref __self_0_0) => true && (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &MyId) -> bool {
        match *__arg_0 {
            MyId(ref __self_1_0) => match *self {
                MyId(ref __self_0_0) => false || (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for MyId {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<i32>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::hash::Hash for MyId {
    fn hash<__H: ::std::hash::Hasher>(&self, __arg_0: &mut __H) -> () {
        match *self {
            MyId(ref __self_0_0) => ::std::hash::Hash::hash(&(*__self_0_0), __arg_0),
        }
    }
}
const _IMPL_DIESEL_NEW_TYPE_FOR_MYID: () = {
    extern crate diesel;
    impl diesel::types::HasSqlType<IdType> for diesel::sqlite::Sqlite {
        fn metadata(_: &()) -> diesel::sqlite::SqliteType {
            diesel::sqlite::SqliteType::Integer
        }
    }
    impl<DB: diesel::backend::Backend> diesel::types::ToSql<IdType, DB> for MyId
    where
        DB: diesel::types::HasSqlType<IdType>,
        DB: diesel::types::HasSqlType<i32>,
        i32: diesel::types::ToSql<i32, DB>,
    {
        fn to_sql<W: ::std::io::Write>(
            &self,
            out: &mut diesel::types::ToSqlOutput<W, DB>,
        ) -> Result<diesel::types::IsNull, Box<::std::error::Error + Send + Sync>> {
            self.0.to_sql(out)
        }
    }
    impl diesel::expression::Expression for IdType {
        type SqlType = diesel::types::Integer;
    }
    impl diesel::expression::AsExpression<IdType> for MyId
    where
        diesel::expression::bound::Bound<IdType, i32>: diesel::expression::Expression<SqlType = IdType>,
    {
        type Expression = diesel::expression::bound::Bound<IdType, i32>;
        fn as_expression(self) -> Self::Expression {
            diesel::expression::bound::Bound::new(self.0)
        }
    }
    impl<'expr, IdType> diesel::expression::AsExpression<IdType> for &'expr MyId
    where
        diesel::expression::bound::Bound<IdType, i32>: diesel::expression::Expression<SqlType = IdType>,
    {
        type Expression = diesel::expression::bound::Bound<IdType, &'expr i32>;
        fn as_expression(self) -> Self::Expression {
            diesel::expression::bound::Bound::new(&self.0)
        }
    }
    impl<DB> diesel::types::FromSql<IdType, DB> for i32
    where
        i32: diesel::types::FromSql<diesel::types::Integer, DB>,
        DB: diesel::backend::Backend,
        DB: diesel::types::HasSqlType<IdType>,
    {
        fn from_sql(
            raw: Option<&<DB as diesel::backend::Backend>::RawValue>,
        ) -> Result<Self, Box<::std::error::Error + Send + Sync>> {
            diesel::types::FromSql::<diesel::types::Integer, DB>::from_sql(raw)
        }
    }
    impl<DB> diesel::types::FromSql<IdType, DB> for MyId
    where
        i32: diesel::types::FromSql<IdType, DB>,
        DB: diesel::backend::Backend,
        DB: diesel::types::HasSqlType<IdType>,
    {
        fn from_sql(
            raw: Option<&<DB as diesel::backend::Backend>::RawValue>,
        ) -> Result<Self, Box<::std::error::Error + Send + Sync>> {
            diesel::types::FromSql::<IdType, DB>::from_sql(raw).map(MyId)
        }
    }
    impl<DB> diesel::types::FromSqlRow<IdType, DB> for MyId
    where
        i32: diesel::types::FromSql<IdType, DB>,
        DB: diesel::backend::Backend,
        DB: diesel::types::HasSqlType<IdType>,
    {
        fn build_from_row<R: diesel::row::Row<DB>>(
            row: &mut R,
        ) -> Result<Self, Box<::std::error::Error + Send + Sync>> {
            diesel::types::FromSql::<IdType, DB>::from_sql(row.take())
        }
    }
    impl<DB> diesel::query_source::Queryable<IdType, DB> for MyId
    where
        i32: diesel::types::FromSqlRow<IdType, DB>,
        DB: diesel::backend::Backend,
        DB: diesel::types::HasSqlType<IdType>,
    {
        type Row = i32;
        fn build(row: Self::Row) -> Self {
            MyId(row)
        }
    }
    impl diesel::query_builder::QueryId for MyId {
        type QueryId = Self;
        fn has_static_query_id() -> bool {
            true
        }
    }
    impl diesel::query_builder::QueryId for IdType {
        type QueryId = Self;
        fn has_static_query_id() -> bool {
            true
        }
    }
    impl diesel::query_builder::Query for IdType {
        type SqlType = IdType;
    }
    impl<DB> diesel::query_builder::QueryFragment<DB> for IdType
    where
        DB: diesel::backend::Backend,
        i32: diesel::query_builder::QueryFragment<DB>,
        diesel::types::Integer: diesel::query_builder::QueryFragment<DB>,
    {
        fn walk_ast(&self, mut out: diesel::query_builder::AstPass<DB>) -> QueryResult<()> {
            diesel::types::Integer.walk_ast(out)
        }
    }
    impl<DB> diesel::query_builder::QueryFragment<DB> for MyId
    where
        DB: diesel::backend::Backend,
        i32: diesel::query_builder::QueryFragment<DB>,
        IdType: diesel::query_builder::QueryFragment<DB>,
    {
        fn walk_ast(&self, mut out: diesel::query_builder::AstPass<DB>) -> QueryResult<()> {
            IdType.walk_ast(out)
        }
    }
};

#[table_name = "my_entities"]
pub struct MyEntity {
    id: MyId,
    val: i32,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for MyEntity {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MyEntity {
                id: ref __self_0_0,
                val: ref __self_0_1,
            } => {
                let mut builder = __arg_0.debug_struct("MyEntity");
                let _ = builder.field("id", &&(*__self_0_0));
                let _ = builder.field("val", &&(*__self_0_1));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for MyEntity {
    #[inline]
    fn clone(&self) -> MyEntity {
        match *self {
            MyEntity {
                id: ref __self_0_0,
                val: ref __self_0_1,
            } => MyEntity {
                id: ::std::clone::Clone::clone(&(*__self_0_0)),
                val: ::std::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for MyEntity {
    #[inline]
    fn eq(&self, __arg_0: &MyEntity) -> bool {
        match *__arg_0 {
            MyEntity {
                id: ref __self_1_0,
                val: ref __self_1_1,
            } => match *self {
                MyEntity {
                    id: ref __self_0_0,
                    val: ref __self_0_1,
                } => true && (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, __arg_0: &MyEntity) -> bool {
        match *__arg_0 {
            MyEntity {
                id: ref __self_1_0,
                val: ref __self_1_1,
            } => match *self {
                MyEntity {
                    id: ref __self_0_0,
                    val: ref __self_0_1,
                } => false || (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}





macro_rules! __static_cond(( id id ) => {
                           __diesel_fields_with_field_names ! {
                           (
                           targets = (  ) , fields = [
                           {
                           field_name : id , column_name : id , field_ty :
                           MyId , field_kind : regular , inner_field_ty : MyId
                           , } {
                           field_name : val , column_name : val , field_ty :
                           i32 , field_kind : regular , inner_field_ty : i32 ,
                           } ] , headers = (
                           table_name = my_entities , struct_ty = MyEntity ,
                           lifetimes = (  ) , ) , callback = impl_Identifiable
                           , found_fields = [  ] , ) ,
                           found_field_with_field_name = id , field = {
                           field_name : id , column_name : id , field_ty :
                           MyId , field_kind : regular , inner_field_ty : MyId
                           , } , } } ; ( id id ) => {  });
impl ::associations::HasTable for MyEntity {
    type Table = my_entities::table;
    fn table() -> Self::Table {
        my_entities::table
    }
}
impl<'ident> ::associations::Identifiable for &'ident MyEntity {
    type Id = (&'ident MyId);
    fn id(self) -> Self::Id {
        (&self.id)
    }
}
macro_rules! __static_cond(( id id ) => {
                           __diesel_fields_with_field_names ! {
                           (
                           targets = (  ) , fields = [
                           {
                           field_name : id , column_name : id , field_ty :
                           MyId , field_kind : regular , inner_field_ty : MyId
                           , } {
                           field_name : val , column_name : val , field_ty :
                           i32 , field_kind : regular , inner_field_ty : i32 ,
                           } ] , headers = (
                           table_name = my_entities , struct_ty = MyEntity ,
                           lifetimes = (  ) , ) , callback = impl_Identifiable
                           , found_fields = [  ] , ) ,
                           found_field_with_field_name = val , field = {
                           field_name : val , column_name : val , field_ty :
                           i32 , field_kind : regular , inner_field_ty : i32 ,
                           } , } } ; ( id val ) => {  });
impl <'insert, DB> ::insertable::Insertable<my_entities::table, DB> for
 &'insert MyEntity where DB: ::backend::Backend,
 (::insertable::ColumnInsertValue<my_entities::id,
                                  ::expression::helper_types::AsExpr<&'insert MyId,
                                                                     my_entities::id>>,
  ::insertable::ColumnInsertValue<my_entities::val,
                                  ::expression::helper_types::AsExpr<&'insert i32,
                                                                     my_entities::val>>): ::insertable::InsertValues<DB>
 {
    type
    Values
    =
    (::insertable::ColumnInsertValue<my_entities::id,
                                     ::expression::helper_types::AsExpr<&'insert MyId,
                                                                        my_entities::id>>,
     ::insertable::ColumnInsertValue<my_entities::val,
                                     ::expression::helper_types::AsExpr<&'insert i32,
                                                                        my_entities::val>>);
    #[allow(non_shorthand_field_patterns)]
    fn values(self) -> Self::Values {
        use ::expression::{AsExpression, Expression};
        use ::insertable::ColumnInsertValue;
        let MyEntity { id: ref id, val: ref val } = *self;
        (ColumnInsertValue::Expression(my_entities::id,
                                       AsExpression::<<my_entities::id as
                                                      Expression>::SqlType>::as_expression(id)),
         ColumnInsertValue::Expression(my_entities::val,
                                       AsExpression::<<my_entities::val as
                                                      Expression>::SqlType>::as_expression(val)))
    }
}
impl<'insert, Op> ::query_builder::insert_statement::IntoInsertStatement<my_entities::table, Op>
    for &'insert MyEntity {
    type InsertStatement = ::query_builder::insert_statement::InsertStatement<
        my_entities::table,
        Self,
        Op,
    >;
    fn into_insert_statement(
        self,
        target: my_entities::table,
        operator: Op,
    ) -> Self::InsertStatement {
        ::query_builder::insert_statement::InsertStatement::no_returning_clause(
            target,
            self,
            operator,
        )
    }
}
impl<'insert> ::query_builder::insert_statement::UndecoratedInsertRecord<my_entities::table>
    for &'insert MyEntity {
}
const _IMPL_QUERYABLE_FOR_MYENTITY: () = {
    extern crate diesel;
    impl<__DB, __ST> diesel::Queryable<__ST, __DB> for MyEntity
    where
        __DB: diesel::backend::Backend + diesel::types::HasSqlType<__ST>,
        (MyId, i32): diesel::types::FromSqlRow<__ST, __DB>,
    {
        type Row = (MyId, i32);
        fn build((id, val): Self::Row) -> Self {
            MyEntity { id: id, val: val }
        }
    }
};
pub mod my_entities {
    #![allow(dead_code)]
    use {JoinTo, QuerySource, Table};
    use associations::HasTable;
    use query_builder::*;
    use query_builder::nodes::Identifier;
    use query_source::{AppearsInFromClause, Never, Once};
    use query_source::joins::PleaseGenerateInverseJoinImpls;
    use types::*;
    pub use self::columns::*;
    /// Re-exports all of the columns of this table, as well as the
    /// table struct renamed to the module name. This is meant to be
    /// glob imported for functions which only deal with one table.
    pub mod dsl {
        pub use super::columns::{id, val};
        pub use super::table as my_entities;
    }
    #[allow(non_upper_case_globals, dead_code)]
    /// A tuple of all of the columns on this table
    pub const all_columns: (id, val) = (id, val);
    #[allow(non_camel_case_types)]
    /// The actual table struct
    ///
    /// This is the type which provides the base methods of the query
    /// builder, such as `.select` and `.filter`.
    #[rustc_copy_clone_marker]
    pub struct table;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::std::fmt::Debug for table {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                table => {
                    let mut builder = __arg_0.debug_tuple("table");
                    builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::std::clone::Clone for table {
        #[inline]
        fn clone(&self) -> table {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::std::marker::Copy for table {}
    impl table {
        #[allow(dead_code)]
        /// Represents `table_name.*`, which is sometimes necessary
        /// for efficient count queries. It cannot be used in place of
        /// `all_columns`
        pub fn star(&self) -> star {
            star
        }
    }
    /// The SQL type of all of the columns on this table
    pub type SqlType = (::IdType, Integer);
    /// Helper type for reperesenting a boxed query from this table
    pub type BoxedQuery<'a, DB, ST = SqlType> = BoxedSelectStatement<'a, ST, table, DB>;
    impl QuerySource for table {
        type FromClause = Identifier<'static>;
        type DefaultSelection = <Self as Table>::AllColumns;
        fn from_clause(&self) -> Self::FromClause {
            Identifier("my_entities")
        }
        fn default_selection(&self) -> Self::DefaultSelection {
            Self::all_columns()
        }
    }
    impl AsQuery for table {
        type SqlType = SqlType;
        type Query = SelectStatement<Self>;
        fn as_query(self) -> Self::Query {
            SelectStatement::simple(self)
        }
    }
    impl Table for table {
        type PrimaryKey = columns::id;
        type AllColumns = (id, val);
        fn primary_key(&self) -> Self::PrimaryKey {
            columns::id
        }
        fn all_columns() -> Self::AllColumns {
            (id, val)
        }
    }
    impl HasTable for table {
        type Table = Self;
        fn table() -> Self::Table {
            table
        }
    }
    impl IntoUpdateTarget for table {
        type
        WhereClause
        =
        <<Self as AsQuery>::Query as IntoUpdateTarget>::WhereClause;
        fn into_update_target(self) -> UpdateTarget<Self::Table, Self::WhereClause> {
            self.as_query().into_update_target()
        }
    }
    impl AppearsInFromClause<table> for table {
        type Count = Once;
    }
    impl<T> AppearsInFromClause<T> for table
    where
        T: Table + JoinTo<table>,
    {
        type Count = Never;
    }
    impl<T> JoinTo<T> for table
    where
        T: JoinTo<table> + JoinTo<PleaseGenerateInverseJoinImpls<table>>,
    {
        type FromClause = T;
        type OnClause = <T as JoinTo<table>>::OnClause;
        fn join_target(rhs: T) -> (Self::FromClause, Self::OnClause) {
            let (_, on_clause) = T::join_target(table);
            (rhs, on_clause)
        }
    }
    impl ::query_builder::QueryId for table {
        type QueryId = Self;
        fn has_static_query_id() -> bool {
            true
        }
    }
    /// Contains all of the columns of this table
    pub mod columns {
        use super::table;
        use {AppearsOnTable, Expression, QuerySource, SelectableExpression};
        use backend::Backend;
        use query_builder::{AstPass, QueryFragment, SelectStatement};
        use query_source::joins::{Inner, Join, JoinOn, LeftOuter};
        use query_source::{AppearsInFromClause, Never, Once};
        use result::QueryResult;
        use types::*;
        #[allow(non_camel_case_types, dead_code)]
        /// Represents `table_name.*`, which is sometimes needed for
        /// efficient count queries. It cannot be used in place of
        /// `all_columns`, and has a `SqlType` of `()` to prevent it
        /// being used that way
        #[rustc_copy_clone_marker]
        pub struct star;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::fmt::Debug for star {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    star => {
                        let mut builder = __arg_0.debug_tuple("star");
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::clone::Clone for star {
            #[inline]
            fn clone(&self) -> star {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::marker::Copy for star {}
        impl Expression for star {
            type SqlType = ();
        }
        impl<DB: Backend> QueryFragment<DB> for star
        where
            <table as QuerySource>::FromClause: QueryFragment<DB>,
        {
            fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
                table.from_clause().walk_ast(out.reborrow())?;
                out.push_sql(".*");
                Ok(())
            }
        }
        impl SelectableExpression<table> for star {}
        impl AppearsOnTable<table> for star {}
        #[allow(non_camel_case_types, dead_code)]
        #[rustc_copy_clone_marker]
        pub struct id;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::fmt::Debug for id {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    id => {
                        let mut builder = __arg_0.debug_tuple("id");
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::clone::Clone for id {
            #[inline]
            fn clone(&self) -> id {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::marker::Copy for id {}
        impl ::expression::Expression for id {
            type SqlType = ::IdType;
        }
        impl<DB> ::query_builder::QueryFragment<DB> for id
        where
            DB: ::backend::Backend,
            <table as QuerySource>::FromClause: QueryFragment<DB>,
        {
            fn walk_ast(&self, mut out: ::query_builder::AstPass<DB>) -> ::result::QueryResult<()> {
                table.from_clause().walk_ast(out.reborrow())?;
                out.push_sql(".");
                out.push_identifier("id")
            }
        }
        impl ::query_builder::QueryId for id {
            type QueryId = Self;
            fn has_static_query_id() -> bool {
                true
            }
        }
        impl SelectableExpression<table> for id {}
        impl<QS> AppearsOnTable<QS> for id
        where
            QS: AppearsInFromClause<table, Count = Once>,
        {
        }
        impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for id
        where
            id: AppearsOnTable<Join<Left, Right, LeftOuter>>,
            Left: AppearsInFromClause<table, Count = Once>,
            Right: AppearsInFromClause<table, Count = Never>,
        {
        }
        impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for id
        where
            id: AppearsOnTable<Join<Left, Right, Inner>>,
            Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
        {
        }
        impl<Join, On> SelectableExpression<JoinOn<Join, On>> for id
        where
            id: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>,
        {
        }
        impl<From> SelectableExpression<SelectStatement<From>> for id
        where
            id: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>,
        {
        }
        impl ::expression::NonAggregate for id {}
        impl ::query_source::Column for id {
            type Table = table;
            fn name() -> &'static str {
                "id"
            }
        }
        impl<T> ::EqAll<T> for id
        where
            T: ::expression::AsExpression<::IdType>,
            ::expression::helper_types::Eq<id, T>: ::Expression<SqlType = ::types::Bool>,
        {
            type Output = ::expression::helper_types::Eq<Self, T>;
            fn eq_all(self, rhs: T) -> Self::Output {
                ::expression::operators::Eq::new(self, rhs.as_expression())
            }
        }
        #[allow(non_camel_case_types, dead_code)]
        #[rustc_copy_clone_marker]
        pub struct val;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::fmt::Debug for val {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    val => {
                        let mut builder = __arg_0.debug_tuple("val");
                        builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::clone::Clone for val {
            #[inline]
            fn clone(&self) -> val {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types, dead_code)]
        impl ::std::marker::Copy for val {}
        impl ::expression::Expression for val {
            type SqlType = Integer;
        }
        impl<DB> ::query_builder::QueryFragment<DB> for val
        where
            DB: ::backend::Backend,
            <table as QuerySource>::FromClause: QueryFragment<DB>,
        {
            fn walk_ast(&self, mut out: ::query_builder::AstPass<DB>) -> ::result::QueryResult<()> {
                table.from_clause().walk_ast(out.reborrow())?;
                out.push_sql(".");
                out.push_identifier("val")
            }
        }
        impl ::query_builder::QueryId for val {
            type QueryId = Self;
            fn has_static_query_id() -> bool {
                true
            }
        }
        impl SelectableExpression<table> for val {}
        impl<QS> AppearsOnTable<QS> for val
        where
            QS: AppearsInFromClause<table, Count = Once>,
        {
        }
        impl<Left, Right> SelectableExpression<Join<Left, Right, LeftOuter>> for val
        where
            val: AppearsOnTable<Join<Left, Right, LeftOuter>>,
            Left: AppearsInFromClause<table, Count = Once>,
            Right: AppearsInFromClause<table, Count = Never>,
        {
        }
        impl<Left, Right> SelectableExpression<Join<Left, Right, Inner>> for val
        where
            val: AppearsOnTable<Join<Left, Right, Inner>>,
            Join<Left, Right, Inner>: AppearsInFromClause<table, Count = Once>,
        {
        }
        impl<Join, On> SelectableExpression<JoinOn<Join, On>> for val
        where
            val: SelectableExpression<Join> + AppearsOnTable<JoinOn<Join, On>>,
        {
        }
        impl<From> SelectableExpression<SelectStatement<From>> for val
        where
            val: SelectableExpression<From> + AppearsOnTable<SelectStatement<From>>,
        {
        }
        impl ::expression::NonAggregate for val {}
        impl ::query_source::Column for val {
            type Table = table;
            fn name() -> &'static str {
                "val"
            }
        }
        impl<T> ::EqAll<T> for val
        where
            T: ::expression::AsExpression<Integer>,
            ::expression::helper_types::Eq<val, T>: ::Expression<SqlType = ::types::Bool>,
        {
            type Output = ::expression::helper_types::Eq<Self, T>;
            fn eq_all(self, rhs: T) -> Self::Output {
                ::expression::operators::Eq::new(self, rhs.as_expression())
            }
        }
        impl <Rhs> ::std::ops::Add<Rhs> for val where
         Rhs: ::expression::AsExpression<<<val as ::Expression>::SqlType as
                                         ::types::ops::Add>::Rhs> {
            type
            Output
            =
            ::expression::ops::Add<Self, Rhs::Expression>;
            fn add(self, rhs: Rhs) -> Self::Output {
                ::expression::ops::Add::new(self, rhs.as_expression())
            }
        }
        impl <Rhs> ::std::ops::Sub<Rhs> for val where
         Rhs: ::expression::AsExpression<<<val as ::Expression>::SqlType as
                                         ::types::ops::Sub>::Rhs> {
            type
            Output
            =
            ::expression::ops::Sub<Self, Rhs::Expression>;
            fn sub(self, rhs: Rhs) -> Self::Output {
                ::expression::ops::Sub::new(self, rhs.as_expression())
            }
        }
        impl <Rhs> ::std::ops::Div<Rhs> for val where
         Rhs: ::expression::AsExpression<<<val as ::Expression>::SqlType as
                                         ::types::ops::Div>::Rhs> {
            type
            Output
            =
            ::expression::ops::Div<Self, Rhs::Expression>;
            fn div(self, rhs: Rhs) -> Self::Output {
                ::expression::ops::Div::new(self, rhs.as_expression())
            }
        }
        impl <Rhs> ::std::ops::Mul<Rhs> for val where
         Rhs: ::expression::AsExpression<<<val as ::Expression>::SqlType as
                                         ::types::ops::Mul>::Rhs> {
            type
            Output
            =
            ::expression::ops::Mul<Self, Rhs::Expression>;
            fn mul(self, rhs: Rhs) -> Self::Output {
                ::expression::ops::Mul::new(self, rhs.as_expression())
            }
        }
    }
}
#[cfg(test)]
fn setup() -> SqliteConnection {
    let conn = SqliteConnection::establish(":memory:").unwrap();
    let setup = sql::<diesel::types::Bool>(
        "CREATE TABLE IF NOT EXISTS my_entities (\n                id INTEGER PRIMARY KEY,\n                val INTEGER\n         )",
    );
    setup.execute(&conn).expect("Can\'t create table");
    conn
}
#[test]
pub fn does_roundtrip() {
    let conn = setup();
    let obj = MyEntity {
        id: MyId(0),
        val: 1,
    };
    ExecuteDsl::execute(diesel::insert(&obj).into(my_entities::table), &conn)
        .expect("Couldn\'t insert struct into my_entities");
    let found: Vec<MyEntity> = my_entities::table.load(&conn).unwrap();
    ::io::_print(::std::fmt::Arguments::new_v1(
        {
            static __STATIC_FMTSTR: &'static [&'static str] = &["found: ", "\n"];
            __STATIC_FMTSTR
        },
        &match (&found,) {
            (__arg0,) => [::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Debug::fmt)],
        },
    ));
    {
        match (&found[0], &obj) {
            (left_val, right_val) => if !(*left_val == *right_val) {
                {
                    ::rt::begin_panic_fmt(
                        &::std::fmt::Arguments::new_v1(
                            {
                                static __STATIC_FMTSTR: &'static [&'static str] = &[
                                    "assertion failed: `(left == right)`\n  left: `",
                                    "`,\n right: `",
                                    "`",
                                ];
                                __STATIC_FMTSTR
                            },
                            &match (&left_val, &right_val) {
                                (__arg0, __arg1) => [
                                    ::std::fmt::ArgumentV1::new(__arg0, ::std::fmt::Debug::fmt),
                                    ::std::fmt::ArgumentV1::new(__arg1, ::std::fmt::Debug::fmt),
                                ],
                            },
                        ),
                        {
                            static _FILE_LINE_COL: (&'static str, u32) =
                                ("tests/domain-type.rs", 53u32);
                            &_FILE_LINE_COL
                        },
                    )
                }
            },
        }
    };
}
pub mod __test_reexports {
    pub use super::does_roundtrip;
}
pub mod __test {
    extern crate test;
    #[main]
    pub fn main() -> () {
        test::test_main_static(TESTS)
    }
    const TESTS: &'static [self::test::TestDescAndFn] = &[
        self::test::TestDescAndFn {
            desc: self::test::TestDesc {
                name: self::test::StaticTestName("does_roundtrip"),
                ignore: false,
                should_panic: self::test::ShouldPanic::No,
                //allow_fail: false,
            },
            testfn: self::test::StaticTestFn(::__test_reexports::does_roundtrip),
        },
    ];
}
