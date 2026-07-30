//! The collector awaits every test, so a blocking one cannot be collected.
use test_utils_derive::{trait_test, trait_test_suite};

#[trait_test_suite]
mod suite {
    use super::*;

    #[trait_test]
    fn blocking(subject: &u8) {
        let _ = subject;
    }
}

fn main() {}
