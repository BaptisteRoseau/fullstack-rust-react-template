//! A concrete subject pins the suite to one backend, defeating its purpose.
use test_trait_derive::{test_trait, test_trait_suite};

struct MyBackend;

#[test_trait_suite]
mod suite {
    use super::*;

    #[test_trait]
    async fn takes_a_backend(subject: &MyBackend) {
        let _ = subject;
    }
}

fn main() {}
