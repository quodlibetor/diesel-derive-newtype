use diesel_derive_newtype::DieselNewType;

#[derive(DieselNewType)]
struct Named {
    value: i64,
}

fn main() {}
