//! Copy to `crates/<mycrate>/tests/backends/<backend>.rs` for a backend that
//! needs a real service. Replace the names. Delete this header, keep the first
//! line as the `//!` doc. Order of items below is the convention — keep it.

//! Runs the `MyTrait` trait suite against the `SomeBackend` backend.

use std::sync::Arc;

use testcontainers::core::ContainerPort::Tcp;
use testcontainers::core::WaitFor;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, runners::AsyncRunner};

use mycrate::backends::SomeBackend;
use test_trait::{Runtime, TestSuite, Trial};

#[path = "../trait_tests.rs"]
mod trait_tests;

test_trait::test_trait_main!(SomeBackendFixture);

struct SomeBackendFixture {
    _container: ContainerAsync<GenericImage>,
    url: String,
}

impl TestSuite for SomeBackendFixture {
    async fn start() -> Self {
        Self::start_container().await
    }

    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        trait_tests::suite::trials(rt, move || {
            let fixture = self.clone();
            async move {
                SomeBackend::try_new(&fixture.url).expect("failed to create SomeBackend")
            }
        })
    }
}

impl SomeBackendFixture {
    async fn start_container() -> Self {
        let container = GenericImage::new(IMAGE, TAG)
            .with_exposed_port(Tcp(PORT))
            .with_wait_for(WaitFor::message_on_stdout("ready to accept connections"))
            .with_copy_to("/etc/service.toml", SERVICE_CONFIG.as_bytes().to_vec())
            .start()
            .await
            .expect("failed to start someservice container");

        let port = container
            .get_host_port_ipv4(PORT)
            .await
            .expect("failed to get someservice port");

        Self {
            _container: container,
            url: format!("http://127.0.0.1:{port}"),
        }
    }
}

const IMAGE: &str = "someorg/someservice";
/// Pinned: the tests drive this service's wire format.
const TAG: &str = "1.2.3";
const PORT: u16 = 1234;

const SERVICE_CONFIG: &str = include_str!("../assets/service.toml");
