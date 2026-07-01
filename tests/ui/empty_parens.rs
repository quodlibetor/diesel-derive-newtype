use diesel_derive_newtype::DieselNewType;

#[derive(DieselNewType)]
#[diesel_newtype()]
struct EmptyParens(i64);

fn main() {}
