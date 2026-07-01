use diesel_derive_newtype::DieselNewType;

#[derive(DieselNewType)]
#[diesel_newtype(try_from = i64, try_from = i64)]
struct DupTryFrom(i64);

fn main() {}
