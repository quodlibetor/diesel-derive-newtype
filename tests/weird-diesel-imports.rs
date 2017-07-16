#[macro_use] extern crate diesel as gasoline;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel_newtype;

use gasoline::prelude::*;
use gasoline::sqlite::SqliteConnection;

#[derive(Debug, PartialEq, Eq, Hash, DieselNewType)]
pub struct MyId(String);

#[derive(Debug, PartialEq, Identifiable, Queryable)]
#[table_name="my_entities"]
pub struct MyEntity {
    id: MyId,
}

table! {
    my_entities {
        id -> Text,
    }
}

#[allow(dead_code)]
fn from_sql() {
    let conn = SqliteConnection::establish("test.db").unwrap();
    let _: Vec<MyEntity> = my_entities::table.load(&conn).unwrap();
}

#[allow(dead_code)]
#[cfg(feature = "currently-broken")]
fn queryable() {
    let conn = SqliteConnection::establish("test.db").unwrap();
    let _: Vec<MyId> = my_entities::table.select(my_entities::columns::id).load(&conn).unwrap();
}
