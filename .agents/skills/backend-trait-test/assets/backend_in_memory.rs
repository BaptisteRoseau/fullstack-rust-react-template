//! Copy to `crates/<mycrate>/tests/backends/<backend>.rs` for a backend that
//! starts no service (an in-process backend, or a `testing::Mock*` double).
//! Replace the names. Delete this header, keep the first line as the `//!` doc.

//! Runs the `MyTrait` trait suite against the `SomeBackend` backend.

use std::sync::Arc;

use mycrate::backends::SomeBackend;
use test_trait::{Runtime, TestSuite, Trial};

#[path = "../trait_tests.rs"]
mod trait_tests;

test_trait::test_trait_main!(SomeBackendFixture);

struct SomeBackendFixture;

impl TestSuite for SomeBackendFixture {
    async fn start() -> Self {
        Self
    }

    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        trait_tests::suite::trials(rt, || async { SomeBackend::default() })
    }
}
