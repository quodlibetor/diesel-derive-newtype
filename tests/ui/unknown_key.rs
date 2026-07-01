use diesel_derive_newtype::DieselNewType;

#[derive(DieselNewType)]
#[diesel_newtype(bogus = i64)]
struct UnknownKey(i64);

fn main() {}
