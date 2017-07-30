# `#[derive(DieselNewType)]`

Prototype to support newtypes inside of Diesel.

[![Build Status](https://travis-ci.org/quodlibetor/diesel-newtype.svg?branch=master)](https://travis-ci.org/quodlibetor/diesel-newtype)

## What it does

This exposes a single custom-derive macro `DieselNewType` which implements
`ToSql`, `FromSql`, `FromSqlRow`, and `QueryId` for the single-field tuple
struct it is applied to.

This should be enough for you to use newtypes anywhere you would use their
underlying types within Diesel.

### Example

```rust
#[derive(DieselNewType)] // Doesn't need to be on its own line
#[derive(Debug)] // required for DieselNewType
#[derive(Hash, PartialEq, Eq)] // required for other diesel items
struct MyId(i64);

#[derive(Debug, PartialEq, Identifiable, Queryable, Associations)]
pub struct MyEntity {
    id: MyId,
    something_important: u8,
}
```

Oooohhh. Ahhhh.

### Using it

Put this in your Cargo.toml:

```toml
[dependencies]
diesel-newtype = { git = "https://github.com/quodlibetor/diesel-newtype" }
```

There'll be a version on crates.io once I've got a bit more confidents sure
that this is fit for purpose.

## Limitations

* I would like to write more tests (that don't require a running database), but
  they're kind of annoying and I can't figure out how to make them work
* I've only been using it for about an hour, and before this I hadn't actually
  tried sticking my newtypes into the DB via Diesel, so there are probably
  unknown unknowns
* Doesn't try to handle generics at all. I haven't encountered generics on
  newtypes so I didn't bother. They should be easy to add
* It seems almost... too... easy...

That said, it does seem to work.

## License

diesel-newtype is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

Patches and bug reports welcome!
