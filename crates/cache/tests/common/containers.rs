use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::redis::{Redis, REDIS_PORT};

pub struct RedisFixture {
    _container: ContainerAsync<Redis>,
    pub url: String,
}

impl RedisFixture {
    pub async fn start() -> Self {
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
}
