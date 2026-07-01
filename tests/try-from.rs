//! Tests for `#[diesel_newtype(try_from = Type)]`, which deserializes the
//! inner type from the database and then `.try_into()`s the newtype so that
//! invariants are upheld when reading (an invalid value can't be constructed
//! from SQL).

use std::convert::TryFrom;
use std::fmt;

use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_derive_newtype::DieselNewType;

/// A newtype whose inner field is private and can only hold even numbers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, DieselNewType)]
#[diesel_newtype(try_from = i32)]
pub struct Even(i32);

#[derive(Debug, PartialEq)]
pub struct NotEven(i32);

impl fmt::Display for NotEven {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not even", self.0)
    }
}

impl std::error::Error for NotEven {}

impl TryFrom<i32> for Even {
    type Error = NotEven;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(Even(value))
        } else {
            Err(NotEven(value))
        }
    }
}

/// A newtype using the bare `#[diesel_newtype(try_from)]` (intermediate type
/// defaults to the field type) *and* an infallible `From` conversion — proving
/// bare-form parsing works and that `try_from` covers the infallible case too
/// (via the std `Into` -> `TryInto` blanket impls).
#[derive(Debug, Clone, PartialEq, Eq, Hash, DieselNewType)]
#[diesel_newtype(try_from)]
pub struct Doubled(i32);

impl From<i32> for Doubled {
    fn from(value: i32) -> Self {
        Doubled(value)
    }
}

table! {
    numbers {
        id -> Integer,
        even -> Integer,
        doubled -> Integer,
    }
}

#[derive(Debug, Clone, PartialEq, Insertable)]
#[diesel(table_name = numbers)]
struct NewNumber {
    id: i32,
    even: Even,
    doubled: Doubled,
}

fn setup() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    let setup = sql::<diesel::sql_types::Bool>(
        "CREATE TABLE numbers (
                id INTEGER PRIMARY KEY,
                even INTEGER NOT NULL,
                doubled INTEGER NOT NULL
         )",
    );
    setup.execute(&mut conn).expect("Can't create table");
    conn
}

fn insert_even(conn: &mut SqliteConnection, id: i32, raw: i32) {
    // Insert a raw i32 directly so we can control (in)validity.
    sql::<diesel::sql_types::Bool>(&format!(
        "INSERT INTO numbers (id, even, doubled) VALUES ({id}, {raw}, {raw})"
    ))
    .execute(conn)
    .expect("insert failed");
}

#[test]
fn valid_value_roundtrips() {
    let mut conn = setup();
    let obj = NewNumber {
        id: 1,
        even: Even(10),
        doubled: Doubled(3),
    };
    diesel::insert_into(numbers::table)
        .values(&obj)
        .execute(&mut conn)
        .expect("insert failed");

    let evens: Vec<Even> = numbers::table
        .select(numbers::even)
        .load(&mut conn)
        .expect("load failed");
    assert_eq!(evens, vec![Even(10)]);
}

#[test]
fn invalid_value_fails_to_deserialize() {
    let mut conn = setup();
    insert_even(&mut conn, 1, 7); // 7 is odd -> Even::try_from must reject it

    let result: QueryResult<Vec<Even>> =
        numbers::table.select(numbers::even).load(&mut conn);

    let err = result.expect_err("odd value must not deserialize into Even");
    assert!(
        err.to_string().contains("is not even"),
        "typed conversion error should be preserved, got: {}",
        err
    );
}

#[test]
fn infallible_from_conversion_works() {
    let mut conn = setup();
    insert_even(&mut conn, 1, 42);

    let doubled: Vec<Doubled> = numbers::table
        .select(numbers::doubled)
        .load(&mut conn)
        .expect("load failed");
    assert_eq!(doubled, vec![Doubled(42)]);
}
