//! Runs the `Cache` trait suite against the `HashMapCache` backend.

use std::sync::Arc;

use cache::backends::hash_map::HashMapCache;
use test_trait::{Runtime, TestSuite, Trial};

#[path = "../trait_tests.rs"]
mod trait_tests;

test_trait::test_trait_main!(HashMapFixture);

struct HashMapFixture;

impl TestSuite for HashMapFixture {
    async fn start() -> Self {
        Self
    }

    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        trait_tests::suite::trials(rt, || async { HashMapCache::default() })
    }
}
