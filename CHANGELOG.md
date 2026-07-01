# Unreleased

# Version 2.1.3

* Add `#[diesel_newtype(try_from = InnerType)]` to build newtypes fallibly when
  reading from the database, so type invariants can be upheld
  ([#12](https://github.com/quodlibetor/diesel-derive-newtype/issues/12)). The
  read path deserializes `InnerType` and calls `.try_into()`; the conversion
  error is preserved (must be convertible into `Box<dyn Error + Send + Sync>`).
  Infallible `From` conversions work too. The write path is unchanged. A bare
  `#[diesel_newtype(try_from)]` is shorthand for `try_from = <field type>`, the
  common case.
* Better error messages: applying the derive to something other than a
  single-field tuple struct (an enum, a named-field struct, wrong field count,
  etc.), or writing an empty `#[diesel_newtype]`/`#[diesel_newtype()]`, now
  produces a clear compile error pointing at the offending code instead of a
  proc-macro panic or an opaque parser error. Named single-field structs are
  now rejected explicitly (they previously emitted code that failed to compile).

# Version 2.1.2

* Fix new non_local_definitions lint in nightly (#31)

# Version 2.1.1

* Add support for structs with internal references to DieselNewTypes (`ethan-lowman-fp` [#30](https://github.com/quodlibetor/diesel-derive-newtype/pull/30)):

  ```rust
  #[derive(DieselNewType)]
  pub struct MyIdString(String); 
 
  #[derive(Insertable, Queryable)]
  #[diesel(table_name = my_entities)]
  pub struct NewMyEntity<'a> {
      id: &'a MyIdString,  // <-- &'a of DieselNewType
  }
  ```

# 2.1.0

* Update for Diesel 2.1 (`@marhag87`), not compatible with Diesel 2.0.x.
* Bump MSRV to 1.65, because that is Diesel's MSRV.

# 2.0.1

* Bind diesel-derive-newtype 2.0.x to Diesel 2.0.x, Diesel 2.1 has trait bounds that are
  non-obvious to make compatible with 2.0.

# 2.0.0

* Support diesel 2.0. diesel-derive-newtype v1.* supports Diesel 1.* and v2.* supports Diesel 2.*

# 1.0.2

* Update syn to 2.0, bump MSRV to 1.56

# 1.0.1

* Update syn/quote/proc-macro2 dependencies to 1.x

# 1.0.0

* Remove non-dev dependency on `diesel` -- `diesel-derive-newtype` generates generic diesel code.
* CI improvements.

# 0.1.1

Bugs Fixed:

* Issue #5: Deriving NewType in the same module as an unnamespaced result
  caused problems. Report and fix by @adwhit


# 0.1.0

Initial release
