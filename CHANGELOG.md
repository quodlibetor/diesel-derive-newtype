# Unreleased

# 2.0.1

* Bind diesel-derive-newtype 2.0.x to diesel 2.0.x, Diesel 2.1 has trait bounds that are
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
