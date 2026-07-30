//! Without a subject there is nothing to run the test against.
use test_utils_derive::{trait_test, trait_test_suite};

#[trait_test_suite]
mod suite {
    use super::*;

    #[trait_test]
    async fn takes_nothing() {}
}

fn main() {}
