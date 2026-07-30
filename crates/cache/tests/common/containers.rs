use std::sync::Arc;

use cache::backends::redis::Redis as RedisBackend;
use test_trait::{Runtime, TestSuite, Trial};
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::redis::{REDIS_PORT, Redis};

pub struct RedisFixture {
    _container: ContainerAsync<Redis>,
    pub url: String,
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

    /// A fresh client per trial: connecting is cheap, and the suite's keys are
    /// namespaced per test anyway.
    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        super::cache::suite::trials(rt, move || {
            let fixture = self.clone();
            async move {
                RedisBackend::new(&fixture.url, None, Some("test".to_string()))
                    .expect("failed to create Redis client")
            }
        })
    }
}
