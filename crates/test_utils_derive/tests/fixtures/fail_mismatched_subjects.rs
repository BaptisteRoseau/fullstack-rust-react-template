//! One suite drives one backend, so its tests must agree on the subject type.
use test_utils_derive::{trait_test, trait_test_suite};

#[trait_test_suite]
mod suite {
    use super::*;

    #[trait_test]
    async fn takes_a_byte(subject: &u8) {
        let _ = subject;
    }

    #[trait_test]
    async fn takes_a_string(subject: &String) {
        let _ = subject;
    }
}

fn main() {}
