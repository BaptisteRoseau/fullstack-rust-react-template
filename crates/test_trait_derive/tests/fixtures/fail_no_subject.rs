//! Without a subject there is nothing to run the test against.
use test_trait_derive::{test_trait, test_trait_suite};

#[test_trait_suite]
mod suite {
    use super::*;

    #[test_trait]
    async fn takes_nothing() {}
}

fn main() {}
