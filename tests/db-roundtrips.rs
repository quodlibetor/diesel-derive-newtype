use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_derive_newtype::DieselNewType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DieselNewType)]
pub struct MyId(String);

#[derive(Debug, Clone, PartialEq, Identifiable, Insertable, Queryable)]
#[diesel(table_name = my_entities)]
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
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    let setup = sql::<diesel::sql_types::Bool>(
        "CREATE TABLE IF NOT EXISTS my_entities (
                id TEXT PRIMARY KEY,
                val Int
         )",
    );
    setup.execute(&mut conn).expect("Can't create table");
    conn
}

#[test]
fn does_roundtrip() {
    let mut conn = setup();
    let obj = MyEntity {
        id: MyId("WooHoo".into()),
        val: 1,
    };

    diesel::insert_into(my_entities::table)
        .values(&obj)
        .execute(&mut conn)
        .expect("Couldn't insert struct into my_entities");

    let found: Vec<MyEntity> = my_entities::table.load(&mut conn).unwrap();
    println!("found: {:?}", found);
    assert_eq!(found[0], obj);
}

#[test]
fn queryable() {
    let mut conn = setup();
    let objs = vec![
        MyEntity {
            id: MyId("WooHoo".into()),
            val: 1,
        },
        MyEntity {
            id: MyId("boo".into()),
            val: 2,
        },
    ];

    diesel::insert_into(my_entities::table)
        .values(&objs)
        .execute(&mut conn)
        .expect("Couldn't insert struct into my_entities");

    let ids: Vec<MyId> = my_entities::table
        .select(my_entities::columns::id)
        .load(&mut conn)
        .unwrap();
    assert_eq!(&ids[0], &objs[0].id);
    assert_eq!(&ids[1], &objs[1].id);
}

#[test]
fn query_as_id() {
    let mut conn = setup();
    let expected = MyEntity {
        id: MyId("WooHoo".into()),
        val: 1,
    };
    let objs = vec![
        MyEntity {
            id: MyId("loop".into()),
            val: 0,
        },
        expected.clone(),
        MyEntity {
            id: MyId("boo".into()),
            val: 2,
        },
    ];

    diesel::insert_into(my_entities::table)
        .values(&objs)
        .execute(&mut conn)
        .expect("Couldn't insert struct into my_entities");

    let ids: Vec<MyEntity> = my_entities::table
        .filter(my_entities::id.eq(MyId("WooHoo".into())))
        .load(&mut conn)
        .unwrap();
    assert_eq!(ids, vec![expected])
}

#[test]
fn query_as_underlying_type() {
    let mut conn = setup();
    let expected = MyEntity {
        id: MyId("WooHoo".into()),
        val: 1,
    };
    let objs = vec![
        MyEntity {
            id: MyId("loop".into()),
            val: 0,
        },
        expected.clone(),
        MyEntity {
            id: MyId("boo".into()),
            val: 2,
        },
    ];

    diesel::insert_into(my_entities::table)
        .values(&objs)
        .execute(&mut conn)
        .expect("Couldn't insert struct into my_entities");

    let ids: Vec<MyEntity> = my_entities::table
        .filter(my_entities::id.eq("WooHoo".to_string()))
        .load(&mut conn)
        .unwrap();
    assert_eq!(ids, vec![expected])
}

#[test]
fn set() {
    let mut conn = setup();
    let expected = MyEntity {
        id: MyId("WooHoo".into()),
        val: 1,
    };
    let objs = vec![
        MyEntity {
            id: MyId("loop".into()),
            val: 0,
        },
        expected.clone(),
        MyEntity {
            id: MyId("boo".into()),
            val: 2,
        },
    ];

    diesel::insert_into(my_entities::table)
        .values(&objs)
        .execute(&mut conn)
        .expect("Couldn't insert struct into my_entities");

    let new_id = MyId("Oh My".into());
    diesel::update(my_entities::table.find(&expected.id))
        .set(my_entities::id.eq(&new_id))
        .execute(&mut conn)
        .unwrap();
    let updated_ids: Vec<MyEntity> = my_entities::table
        .filter(my_entities::id.eq(&new_id))
        .load(&mut conn)
        .unwrap();
    assert_eq!(updated_ids, vec![MyEntity { id: new_id, val: 1 }])
}
