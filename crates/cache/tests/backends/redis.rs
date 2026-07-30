//! Runs the `Cache` trait suite against the `Redis` backend.

use std::sync::Arc;

use cache::backends::redis::Redis as RedisBackend;
use test_trait::{Runtime, TestSuite, Trial};
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::redis::{REDIS_PORT, Redis};

#[path = "../trait_tests.rs"]
mod trait_tests;

test_trait::test_trait_main!(RedisFixture);

struct RedisFixture {
    _container: ContainerAsync<Redis>,
    url: String,
}

impl TestSuite for RedisFixture {
    async fn start() -> Self {
        let container = Redis::default()
            .start()
            .await
            .expect("failed to start redis container");
        let port = container.get_host_port_ipv4(REDIS_PORT).await.unwrap();
        Self {
            _container: container,
            url: format!("redis://127.0.0.1:{port}"),
        }
    }

    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        trait_tests::suite::trials(rt, move || {
            let fixture = self.clone();
            async move {
                RedisBackend::new(&fixture.url, None, Some("test".to_string()))
                    .expect("failed to create Redis client")
            }
        })
    }
}
