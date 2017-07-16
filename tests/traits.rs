#[macro_use]
extern crate diesel_newtype;
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

#[test]
fn can_deserialize() {
    use diesel::types::FromSql;
    use diesel::backend::Backend;
    let val: &<diesel::pg::Pg as Backend>::RawValue = &[0, 0, 0, 0];
    let myid =
        <MyId as FromSql<diesel::types::Integer, diesel::pg::Pg>>::from_sql(Some(val))
        .unwrap();
    assert_eq!(myid, MyId(0));
}
