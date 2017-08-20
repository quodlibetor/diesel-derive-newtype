#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel_newtype;

use diesel::prelude::*;
use diesel::expression::sql;
use diesel::sqlite::SqliteConnection;

struct IdType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DieselNewType)]
#[sql_type(IdType, Integer)]
pub struct MyId(i32);

#[derive(Debug, Clone, PartialEq, Identifiable, Insertable, Queryable)]
#[table_name="my_entities"]
pub struct MyEntity {
    id: MyId,
    val: i32,
}

table! {
    my_entities {
        id -> ::IdType,
        val -> Integer,
    }
}

#[cfg(test)]
fn setup() -> SqliteConnection {
    let conn = SqliteConnection::establish(":memory:").unwrap();
    let setup = sql::<diesel::types::Bool>(
        "CREATE TABLE IF NOT EXISTS my_entities (
                id INTEGER PRIMARY KEY,
                val INTEGER
         )");
    setup.execute(&conn).expect("Can't create table");
    conn
}

#[test]
fn does_roundtrip() {
    let conn = setup();
    let obj = MyEntity { id: MyId(0), val: 1 };

    ExecuteDsl::execute(
        diesel::insert(&obj).into(my_entities::table),
        &conn)
        .expect("Couldn't insert struct into my_entities");

    let found: Vec<MyEntity> = my_entities::table.load(&conn).unwrap();
    println!("found: {:?}", found);
    assert_eq!(found[0], obj);
}
