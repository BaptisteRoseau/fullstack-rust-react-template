//! The collector awaits every test, so a blocking one cannot be collected.
use test_trait_derive::{test_trait, test_trait_suite};

#[test_trait_suite]
mod suite {
    use super::*;

    #[test_trait]
    fn blocking(subject: &u8) {
        let _ = subject;
    }
}

fn main() {}
