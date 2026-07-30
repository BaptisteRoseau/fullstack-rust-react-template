//! Shared scaffolding for the crates that expose a trait and test it against one or
//! more backends.
//!
//! A trait test suite is written once, against the trait, and run against every
//! backend implementing it. [`macro@trait_test_suite`] turns a module of
//! [`macro@trait_test`] functions into the `Vec<Trial>` `libtest-mimic` needs, and
//! [`macro@trait_test_main`] writes the `harness = false` binary's `fn main()` around
//! a [`TestSuite`] fixture.
//!
//! See `crates/cache/tests` for the smallest complete example.

mod suite;

pub use suite::TestSuite;

pub use test_utils_derive::{trait_test, trait_test_main, trait_test_suite};

// Re-exported so a consumer needs one dev-dependency, and so the generated code can
// name these paths without assuming what the consumer has in scope.
pub use async_trait::async_trait;
pub use libtest_mimic::{Arguments, Conclusion, Failed, Trial, run};
pub use tokio::runtime::Runtime;
