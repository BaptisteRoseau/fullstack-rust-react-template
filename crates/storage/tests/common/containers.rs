use s3::{AddressingStyle, Auth, Client, Credentials};
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::minio::MinIO;

pub const TEST_BUCKET: &str = "test-bucket";

pub struct MinioFixture {
    _container: ContainerAsync<MinIO>,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

impl MinioFixture {
    pub async fn start() -> Self {
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

    pub async fn create_bucket(&self, name: &str) {
        let credentials = Credentials::new(&self.access_key, &self.secret_key)
            .expect("invalid credentials");
        let client = Client::builder(&self.endpoint)
            .expect("invalid endpoint")
            .region("us-east-1")
            .auth(Auth::Static(credentials))
            .addressing_style(AddressingStyle::Path)
            .build()
            .expect("failed to build s3 client");
        client
            .buckets()
            .create(name)
            .send()
            .await
            .expect("failed to create test bucket");
    }
}
