//! The macro needs the module body to read its tests.
use test_trait_derive::test_trait_suite;

#[test_trait_suite]
mod suite;

fn main() {}
