//! This is a test file that *DOES* compile and pass tests, but which should
//! not

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel_derive_newtype;

use diesel::prelude::*;
use diesel::expression::sql;
use diesel::sqlite::SqliteConnection;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DieselNewType)]
pub struct MyId(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, DieselNewType)]
pub struct OtherId(String);


#[derive(Debug, Clone, PartialEq, Identifiable, Insertable, Queryable)]
#[table_name="my_entities"]
pub struct MyEntity {
    id: MyId,
    val: i32,
}

table! {
    my_entities {
        id -> Text,
        val -> Integer,
    }
}

#[cfg(test)]
fn setup() -> SqliteConnection {
    let conn = SqliteConnection::establish(":memory:").unwrap();
    let setup = sql::<diesel::types::Bool>(
        "CREATE TABLE IF NOT EXISTS my_entities (
                id TEXT PRIMARY KEY,
                val Int
         )");
    setup.execute(&conn).expect("Can't create table");
    conn
}

#[cfg(test)]
fn setup_with_items() -> (SqliteConnection, Vec<MyEntity>) {
    let conn = setup();
    let objs = vec![
        MyEntity { id: MyId("loop".into()), val: 0 },
        MyEntity { id: MyId("WooHoo".into()), val: 1 },
        MyEntity { id: MyId("boo".into()), val: 2 },
    ];

    diesel::insert(&objs)
        .into(my_entities::table)
        .execute(&conn)
        .expect("Couldn't insert struct into my_entities");

    (conn, objs)
}

#[test]
fn query_as_id() {
    let (conn, _) = setup_with_items();

    let _: Vec<MyEntity> = my_entities::table
        .filter(my_entities::id.eq(OtherId("WooHoo".into()))) // <-- OTHERID
        .load(&conn)
        .unwrap();
}

#[test]
fn set() {
    let (conn, objs) = setup_with_items();

    let expected = objs[1].clone();

    let new_id = OtherId("Oh My".into());  // <-- OTHERID
    diesel::update(my_entities::table.find(&expected.id))
        .set(my_entities::id.eq(&new_id))
        .execute(&conn)
        .unwrap();
}
