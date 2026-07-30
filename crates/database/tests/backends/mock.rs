//! Runs the `Database` trait suite against the `MockDatabase` double.
//!
//! The double is what downstream crates' unit tests run on, so it has to satisfy
//! the same contract as `Postgres` — otherwise those tests pass against behaviour
//! no real backend has.

use std::sync::Arc;

use database::testing::MockDatabase;
use test_trait::{Runtime, TestSuite, Trial};

#[path = "../trait_tests.rs"]
mod trait_tests;

test_trait::test_trait_main!(MockDatabaseFixture);

struct MockDatabaseFixture;

impl TestSuite for MockDatabaseFixture {
    async fn start() -> Self {
        Self
    }

    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        trait_tests::suite::trials(rt, || async { MockDatabase::default() })
    }
}
