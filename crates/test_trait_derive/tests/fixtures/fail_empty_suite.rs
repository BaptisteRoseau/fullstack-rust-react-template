//! A suite with no marked test collects nothing and would run silently.
use test_trait_derive::test_trait_suite;

#[test_trait_suite]
mod suite {
    pub async fn looks_like_a_test(subject: &u8) {
        let _ = subject;
    }
}

fn main() {}
