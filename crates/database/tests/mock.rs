//! Runs the `Database` trait suite against `database::testing::MockDatabase`.
//!
//! The double is what downstream crates' unit tests run on, so it has to satisfy
//! the same contract as `Postgres` — otherwise those tests pass against behaviour
//! no real backend has. No container is involved, so this binary is instant.

use std::sync::Arc;

use database::testing::MockDatabase;
use test_trait::{Runtime, TestSuite, Trial};

#[path = "common/database.rs"]
mod database_suite;

/// Nothing to bring up: the double is in-memory.
struct MockDatabaseFixture;

impl TestSuite for MockDatabaseFixture {
    async fn start() -> Self {
        Self
    }

    /// A fresh `MockDatabase` per trial: the suite's subject is `&mut impl
    /// Database`, so trials cannot share one, and empty `HashMap`s keep them from
    /// colliding on usernames and key hashes.
    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        database_suite::suite::trials(rt, || async { MockDatabase::default() })
    }
}

test_trait::test_trait_main!(MockDatabaseFixture);
