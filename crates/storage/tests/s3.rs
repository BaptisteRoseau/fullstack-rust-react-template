#[macro_use]
mod common;

use std::sync::Arc;

use common::containers::{MinioFixture, TEST_BUCKET};
use libtest_mimic::Arguments;
use storage::backends::S3;

fn main() {
    let args = Arguments::from_args();

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let fixture = Arc::new(rt.block_on(async {
        let f = MinioFixture::start().await;
        f.create_bucket(TEST_BUCKET).await;
        f
    }));

    let make_storage = {
        let fixture = fixture.clone();
        move || {
            S3::try_new(
                &fixture.endpoint,
                TEST_BUCKET,
                &fixture.access_key,
                &fixture.secret_key,
            )
            .expect("failed to create S3 client")
        }
    };

    let tests = storage_trait_tests!(make_storage, rt.clone());

    let conclusion = libtest_mimic::run(&args, tests);

    // Drop fixture inside the tokio runtime context so ContainerAsync::Drop
    // can run its async cleanup.
    let _guard = rt.enter();
    drop(fixture);
    drop(_guard);
    drop(rt);

    conclusion.exit();
}
