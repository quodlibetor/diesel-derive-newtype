use diesel_derive_newtype::DieselNewType;

#[derive(DieselNewType)]
struct TwoFields(i64, i64);

fn main() {}
