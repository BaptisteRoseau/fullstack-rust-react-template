//! One suite drives one backend, so its tests must agree on the subject type.
use test_trait_derive::{test_trait, test_trait_suite};

#[test_trait_suite]
mod suite {
    use super::*;

    #[test_trait]
    async fn takes_a_byte(subject: &u8) {
        let _ = subject;
    }

    #[test_trait]
    async fn takes_a_string(subject: &String) {
        let _ = subject;
    }
}

fn main() {}
