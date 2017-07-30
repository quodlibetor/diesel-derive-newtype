# `#[derive(DieselNewType)]`

Prototype to support newtypes inside of Diesel.

[![Build Status](https://travis-ci.org/quodlibetor/diesel-newtype.svg?branch=master)](https://travis-ci.org/quodlibetor/diesel-newtype)

## What it does

This exposes a single custom-derive macro `DieselNewType` which implements
`ToSql`, `FromSql`, `FromSqlRow`, `Queryable`, `AsExpression` and `QueryId` for
the single-field tuple struct ([NewType][]) it is applied to.

The goal of this project is that you:

* should be enough for you to use newtypes anywhere you would use their
  underlying types within Diesel. (plausibly successful)
* Should get the same compile-time guarantees when using your newtypes as
  expression elements in Diesel as you do in other rust code (not successful,
  see Limitations, below.)

[NewType]: https://aturon.github.io/features/types/newtype.html

### Example

```rust
#[derive(DieselNewType)] // Doesn't need to be on its own line
#[derive(Debug, Hash, PartialEq, Eq)] // required by diesel
struct MyId(i64);

#[derive(Debug, PartialEq, Identifiable, Queryable, Associations)]
pub struct MyEntity {
    id: MyId,
    something_important: u8,
}
```

Oooohhh. Ahhhh.

See [tests/db-roundtrips.rs](tests/db-roundtrips.rs) for a more
complete example.

### Using it

diesel-newtype supports Diesel 0.14 and 0.15. To use Diesel 0.15 put this in
your Cargo.toml:

```toml
[dependencies.diesel-newtype]
git = "https://github.com/quodlibetor/diesel-newtype"
```

For Diesel 0.14 you need to do:

```toml
[dependencies.diesel-newtype]
git = "https://github.com/quodlibetor/diesel-newtype"
default-features = false
features = ["diesel014"]
```

There'll be a version on crates.io once I've got a bit more confidence that
this is fit for purpose.

## Limitations

* It is not as strongly typed as I'd like. Specifically this works and it
  really shouldn't:

    ```rust
    #[derive(Debug, Hash, PartialEq, Eq, DieselNewType)]
    struct OneId(i64);

    #[derive(Debug, Hash, PartialEq, Eq, DieselNewType)]
    struct OtherId(i64);

    #[derive(Debug, Clone, PartialEq, Identifiable, Insertable, Queryable)]
    #[table_name="my_entities"]
    pub struct MyEntity {
        id: OneId,
        val: i32,
    }

    fn darn(conn: &Connection) {
        // shouldn't allow constructing the wrong type, but does
        let OtherId: Vec<OtherId> = my_entities
            .select(id)
            .filter(id.eq(OtherId(1)))  // shouldn't allow filtering by wrong type
            .execute(conn).unwrap();
    }
    ```

  See [`tests/should-not-compile.rs`](tests/should-not-compile.rs) for the
  things I think should fail to compile.

  I believe that the root cause of this is that Diesel implements the various
  expression methods for types that implement `AsExpression`, based on the
  _SQL_ type, not caring about `self` and `other`'s Rust type matching. That
  seems like a pretty good decision in general, but it is a bit unfortunate
  here.

  I hope to find a solution that doesn't involve implementing every
  `*Expression` trait manually with an extra bound, but for now you have to
  keep in mind that the Diesel methods basically auto-transmute your data into
  the underlying SQL type.
* Not battle tested: I've only been using it for a couple weeks, and before
  this I hadn't actually tried sticking my newtypes into the DB via Diesel, so
  there are probably unknown unknowns
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
