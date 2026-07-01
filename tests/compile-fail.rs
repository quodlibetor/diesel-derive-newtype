//! Snapshot tests for the compile errors `#[derive(DieselNewType)]` produces
//! for misuse (wrong shape, malformed attributes). Gated behind the `ui`
//! feature because trybuild `.stderr` snapshots are toolchain-specific; see the
//! `ui` feature comment in Cargo.toml. Run with:
//!
//!   cargo +1.84.0 test --features ui
//!
//! and regenerate snapshots after intentional message changes with:
//!
//!   TRYBUILD=overwrite cargo +1.84.0 test --features ui
#![cfg(feature = "ui")]

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
