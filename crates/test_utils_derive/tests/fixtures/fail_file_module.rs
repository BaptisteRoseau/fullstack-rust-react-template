//! The macro needs the module body to read its tests.
use test_utils_derive::trait_test_suite;

#[trait_test_suite]
mod suite;

fn main() {}
