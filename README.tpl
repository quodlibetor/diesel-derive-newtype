# `diesel-derive-newtype`

Easy-peasy support of newtypes inside of Diesel.

[![Rust](https://github.com/quodlibetor/diesel-derive-newtype/actions/workflows/test.yml/badge.svg?branch=diesel-2)](https://github.com/quodlibetor/diesel-derive-newtype/actions/workflows/test.yml) [![Crates.io Version](https://img.shields.io/crates/v/diesel-derive-newtype.svg)](https://crates.io/crates/diesel-derive-newtype)

### Status

diesel-derive-newtype is small and basically complete -- except for some features
that aren't supported by Diesel (see "limitations", below). Additionally, I don't
use diesel on a daily basis any more, so most maintenance comes from bug reports
or PRs, please open them!

## Installation

diesel-derive-newtype supports Diesel according to its major version -- 0.x
through 1.x support the corresponding diesel versions, 2.0 supports diesel 2.0,
and 2.1 supports 2.1. Generally new versions of diesel-derive-newtype are
released when compilation or tests are observed to fail with a newer version
of diesel, so newer versions of diesel than the released version of
diesel-derive-newtype *may* work. Bug reports and PRs welcome.

New features are only developed for the currently supported version of Diesel.


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


## Releasing

This workflow uses:

* [cargo-readme](https://crates.io/crates/cargo-readme)
* [cargo-release](https://crates.io/crates/cargo-release)

Run, note that we always release patch releases unless diesel has got a new
release:

```
cargo readme > README.md
git diff --exit-code --quiet README.* || (git add README.* && git commit -m "chore: Update README")
cargo release patch
```

## License

diesel-derive-newtype is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

Patches and bug reports welcome!
