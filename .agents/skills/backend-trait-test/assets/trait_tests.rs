//! Copy to `crates/<mycrate>/tests/trait_tests.rs`.
//! Replace `mycrate` / `MyTrait` / `Subject`. Delete this header.

use mycrate::MyTrait;
use test_trait::{test_trait, test_trait_suite};
use uuid::Uuid;

/// Integration tests for the MyTrait trait, run against every backend.
///
/// When adding a test here:
/// - mark it `#[test_trait]` and take the subject as `&impl MyTrait`; the
///   function name becomes the test name, and that is the only place it is written
/// - helpers are unmarked functions, left alone by the macro
#[test_trait_suite]
pub mod suite {
    use super::*;

    #[test_trait]
    async fn does_the_thing(subject: &impl MyTrait) {
        let key = unique_key("does_the_thing");

        let output = subject.do_it(&key).await.expect("do_it failed");

        assert_eq!(output, "expected", "do_it should return the value it stored, got={output:?}");
    }
}

/// Every trial runs in parallel against one service: derive per-test keys.
fn unique_key(test: &str) -> String {
    format!("test-{test}-{}", Uuid::new_v4())
}
