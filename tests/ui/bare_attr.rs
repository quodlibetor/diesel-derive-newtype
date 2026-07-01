use diesel_derive_newtype::DieselNewType;

#[derive(DieselNewType)]
#[diesel_newtype]
struct BareAttr(i64);

fn main() {}
