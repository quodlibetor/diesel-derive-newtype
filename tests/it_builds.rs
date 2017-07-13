#[macro_use]
extern crate diesel_newtype;
#[macro_use]
extern crate diesel;

#[derive(Debug, PartialEq, Eq, DieselNewType)]
struct MyId(i32);

#[test]
fn can_serialize() {
    use diesel::types::ToSql;
    let mut st: Vec<u8> = Vec::new();
    let myid = MyId(0i32);
    <MyId as ToSql<diesel::types::Integer, diesel::sqlite::Sqlite>>::to_sql(&myid, &mut st)
        .unwrap();
    assert_eq!(st, vec![0, 0, 0, 0]);
}

// TODO: figure out some way of testing this without spinning up a DB.
// #[test]
// fn can_deserialize() {
//     use diesel::types::FromSql;
//     use diesel::backend::Backend;
//     let mut st: [u8; 4] = [0, 0, 0, 0];
//     let mut val = <diesel::sqlite::Sqlite as Backend>::RawValue(&st);
//     let myid =
//         <MyId as FromSql<diesel::types::Integer, diesel::sqlite::Sqlite>>::from_sql(Some(&st))
//         .unwrap();
//     assert_eq!(myid, MyId(0));
// }
