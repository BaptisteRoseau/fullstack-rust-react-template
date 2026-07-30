//! A suite with no marked test collects nothing and would run silently.
use test_trait_derive::test_trait_suite;

trait Subject {}

#[test_trait_suite]
mod suite {
    pub async fn looks_like_a_test(subject: &impl Subject) {
        let _ = subject;
    }
}

fn main() {}
