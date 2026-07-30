//! Runs the `Cache` trait suite against `HashMapCache`.
//!
//! It is a real backend and it is also what downstream crates' unit tests run
//! on, so it has to satisfy the same contract as `Redis` — otherwise those tests
//! pass against behaviour no other backend has. No container is involved, so
//! this binary is instant.

use std::sync::Arc;

use cache::backends::hash_map::HashMapCache;
use test_trait::{Runtime, TestSuite, Trial};

#[path = "common/cache.rs"]
mod cache_suite;

/// Nothing to bring up: the backend lives in this process.
struct HashMapFixture;

impl TestSuite for HashMapFixture {
    async fn start() -> Self {
        Self
    }

    /// A fresh `HashMapCache` per trial: it is a `HashMap` behind a mutex, so
    /// building one is free and an empty store keeps parallel trials independent.
    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        cache_suite::suite::trials(rt, || async { HashMapCache::default() })
    }
}

test_trait::test_trait_main!(HashMapFixture);
