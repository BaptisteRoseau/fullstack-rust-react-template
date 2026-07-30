//! One suite drives one trait, so its tests must agree on which.
use test_trait_derive::{test_trait, test_trait_suite};

trait Alpha {}
trait Beta {}

#[test_trait_suite]
mod suite {
    use super::*;

    #[test_trait]
    async fn takes_an_alpha(subject: &impl Alpha) {
        let _ = subject;
    }

    #[test_trait]
    async fn takes_a_beta(subject: &impl Beta) {
        let _ = subject;
    }
}

fn main() {}
