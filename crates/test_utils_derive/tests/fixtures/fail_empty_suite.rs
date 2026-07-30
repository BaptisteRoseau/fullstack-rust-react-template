//! A suite with no marked test collects nothing and would run silently.
use test_utils_derive::trait_test_suite;

#[trait_test_suite]
mod suite {
    pub async fn looks_like_a_test(subject: &u8) {
        let _ = subject;
    }
}

fn main() {}
