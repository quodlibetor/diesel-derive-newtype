# `diesel-derive-newtype`

Easy-peasy support of newtypes inside of Diesel.

[![Rust](https://github.com/quodlibetor/diesel-derive-newtype/actions/workflows/test.yml/badge.svg?branch=diesel-2)](https://github.com/quodlibetor/diesel-derive-newtype/actions/workflows/test.yml) [![Crates.io Version](https://img.shields.io/crates/v/diesel-derive-newtype.svg)](https://crates.io/crates/diesel-derive-newtype)

## Installation

diesel-derive-newtype supports Diesel according to its major version -- 0.x
through 1.x support the corresponding diesel versions, 2.0 supports diesel 2.0,
and 2.1 supports 2.1.

And for Diesel 2.1:

```toml
[dependencies]
diesel-derive-newtype = "2.1.0"
```

for Diesel 2.0.x you have to tell cargo not to upgrade diesel-derive-newtype:

```toml
[dependencies]
diesel-derive-newtype = "~ 2.0.0"
```

And for Diesel 1.x:

```toml
[dependencies]
diesel-derive-newtype = "1.0"
```



{{readme}}

## License

diesel-derive-newtype is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

Patches and bug reports welcome!
