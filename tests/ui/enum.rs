use diesel_derive_newtype::DieselNewType;

#[derive(DieselNewType)]
enum Color {
    Red,
    Green,
}

fn main() {}
