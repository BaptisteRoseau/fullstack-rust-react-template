use testcontainers::core::ContainerPort::Tcp;
use testcontainers::core::{ExecCommand, WaitFor};
use testcontainers::{ContainerAsync, GenericImage, ImageExt, runners::AsyncRunner};

pub const TEST_BUCKET: &str = "test-bucket";

const GARAGE_IMAGE: &str = "dxflrs/garage";
const GARAGE_TAG: &str = "v2.3.0";
const GARAGE_S3_PORT: u16 = 3900;
const GARAGE_KEY_NAME: &str = "test-key";

/// Garage server configuration used by the test container.
///
/// Reuses the deployment manifest so the test backend matches production
/// (notably the `s3_region`, which Garage enforces in request signatures).
const GARAGE_CONFIG: &str = include_str!("../../../../infrastructure/garage/garage.toml");

pub struct GarageFixture {
    container: ContainerAsync<GenericImage>,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

impl GarageFixture {
    pub async fn start() -> Self {
        let container = GenericImage::new(GARAGE_IMAGE, GARAGE_TAG)
            .with_exposed_port(Tcp(GARAGE_S3_PORT))
            .with_wait_for(WaitFor::message_on_stderr("S3 API server listening"))
            .with_cmd(["/garage", "server"])
            .with_copy_to("/etc/garage.toml", GARAGE_CONFIG.as_bytes().to_vec())
            .start()
            .await
            .expect("failed to start garage container");

        let port = container
            .get_host_port_ipv4(GARAGE_S3_PORT)
            .await
            .expect("failed to get garage S3 port");

        // Unlike MinIO, Garage serves no data until a cluster layout is assigned.
        let node_id = exec_stdout(&container, &["/garage", "node", "id", "-q"]).await;
        let node_id = node_id
            .split('@')
            .next()
            .expect("empty garage node id")
            .trim();
        exec(
            &container,
            &[
                "/garage", "layout", "assign", "-z", "dc1", "-c", "1G", node_id,
            ],
        )
        .await;
        exec(
            &container,
            &["/garage", "layout", "apply", "--version", "1"],
        )
        .await;

        // Garage keys are not user/password pairs: create one and read back its
        // generated access key id and secret.
        let key_output =
            exec_stdout(&container, &["/garage", "key", "create", GARAGE_KEY_NAME]).await;
        let (access_key, secret_key) = parse_key_credentials(&key_output);

        Self {
            container,
            endpoint: format!("http://127.0.0.1:{port}"),
            access_key,
            secret_key,
        }
    }

    pub async fn create_bucket(&self, name: &str) {
        exec(&self.container, &["/garage", "bucket", "create", name]).await;
        exec(
            &self.container,
            &[
                "/garage",
                "bucket",
                "allow",
                "--read",
                "--write",
                "--owner",
                name,
                "--key",
                GARAGE_KEY_NAME,
            ],
        )
        .await;
    }
}

/// Runs a command inside the container, discarding its output.
async fn exec(container: &ContainerAsync<GenericImage>, cmd: &[&str]) {
    let _ = exec_stdout(container, cmd).await;
}

/// Runs a command inside the container and returns its captured stdout.
async fn exec_stdout(container: &ContainerAsync<GenericImage>, cmd: &[&str]) -> String {
    let mut result = container
        .exec(ExecCommand::new(cmd.iter().map(|s| s.to_string())))
        .await
        .unwrap_or_else(|e| panic!("failed to exec {cmd:?}: {e}"));
    let stdout = result
        .stdout_to_vec()
        .await
        .unwrap_or_else(|e| panic!("failed to read stdout of {cmd:?}: {e}"));
    String::from_utf8_lossy(&stdout).into_owned()
}

/// Extracts the access key id and secret from `garage key create` output.
fn parse_key_credentials(output: &str) -> (String, String) {
    let mut access_key = None;
    let mut secret_key = None;
    for line in output.lines() {
        if let Some(value) = line.split_once("Key ID:") {
            access_key = Some(value.1.trim().to_string());
        } else if let Some(value) = line.split_once("Secret key:") {
            secret_key = Some(value.1.trim().to_string());
        }
    }
    let access_key = access_key
        .unwrap_or_else(|| panic!("missing Key ID in garage output:\n{output}"));
    let secret_key = secret_key
        .unwrap_or_else(|| panic!("missing Secret key in garage output:\n{output}"));
    (access_key, secret_key)
}
