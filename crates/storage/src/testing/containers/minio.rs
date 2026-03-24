use std::sync::LazyLock;

use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::minio::MinIO;
use tokio::runtime::Runtime;

pub struct MinioFixture {
    _container: ContainerAsync<MinIO>,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

/// Global singleton — one MinIO container shared across all tests.
pub static MINIO: LazyLock<MinioFixture> = LazyLock::new(|| {
    Runtime::new().unwrap().block_on(MinioFixture::start())
});

impl MinioFixture {
    async fn start() -> Self {
        let container = MinIO::default()
            .start()
            .await
            .expect("failed to start minio container");
        let port = container.get_host_port_ipv4(9000).await.unwrap();
        Self {
            _container: container,
            endpoint: format!("http://127.0.0.1:{port}"),
            access_key: "minioadmin".to_string(),
            secret_key: "minioadmin".to_string(),
        }
    }
}
