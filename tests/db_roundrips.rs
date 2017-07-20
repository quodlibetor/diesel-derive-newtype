#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel_newtype;

use diesel::prelude::*;
use diesel::expression::sql;
use diesel::sqlite::SqliteConnection;

#[derive(Debug, PartialEq, Eq, Hash, DieselNewType)]
pub struct MyId(String);

#[derive(Debug, PartialEq, Identifiable, Insertable, Queryable)]
#[table_name="my_entities"]
pub struct MyEntity {
    id: MyId,
}

table! {
    my_entities {
        id -> Text,
    }
}

#[test]
fn does_roundtrip() {
    let conn = SqliteConnection::establish(":memory:").unwrap();
    let setup = sql::<diesel::types::Bool>(
        "CREATE TABLE IF NOT EXISTS my_entities (ID TEXT PRIMARY KEY)");
    setup.execute(&conn).expect("Can't create table");
    let obj = MyEntity { id: MyId("WooHoo".into())};

    diesel::insert(&obj)
        .into(my_entities::table)
        .execute(&conn)
        .expect("Couldn't insert struct into my_entities");
    let found: Vec<MyEntity> = my_entities::table.load(&conn).unwrap();
    println!("found: {:?}", found);
    assert_eq!(found[0], obj);
}

#[allow(dead_code)]
#[cfg(feature = "currently-broken")]
fn queryable() {
    let conn = SqliteConnection::establish(":memory:").unwrap();
    let _: Vec<MyId> = my_entities::table.select(my_entities::columns::id).load(&conn).unwrap();
}
