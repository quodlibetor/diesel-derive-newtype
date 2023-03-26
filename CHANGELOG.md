# Unreleased

# 1.0.2

* Update syn to 2.0, bump MSRV to 1.56

# 1.0.1

* Update syn/quote/proc-macro2 dependencies to 1.x

# 1.0.0

* Remove non-dev dependency on `diesel` -- `diesel-derive-newtype` generates generic diesel code.
  If a future release of Diesel breaks compatibility with `diesel-derive-newtype` an explicit
  dependency on the correct version span will be added and a new release will be made.
* CI improvements.

# 0.1.1

Bugs Fixed:

* Issue #5: Deriving NewType in the same module as an unnamespaced result
  caused problems. Report and fix by @adwhit


# 0.1.0

Initial release
