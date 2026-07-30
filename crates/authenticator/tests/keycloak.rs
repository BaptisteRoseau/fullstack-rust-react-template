#[macro_use]
mod common;

use std::sync::Arc;

use common::containers::KeycloakFixture;
use libtest_mimic::Arguments;

fn main() {
    let args = Arguments::from_args();

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let fixture = Arc::new(rt.block_on(KeycloakFixture::start()));
    let (credentials, bff) = rt.block_on(async {
        (
            Arc::new(fixture.credentials_authenticator().await),
            Arc::new(fixture.bff_authenticator().await),
        )
    });

    let tests = authenticator_trait_tests!(credentials, bff, fixture.clone(), rt.clone());

    let conclusion = libtest_mimic::run(&args, tests);

    // Drop fixture inside the tokio runtime context so ContainerAsync::Drop
    // can run its async cleanup.
    let _guard = rt.enter();
    drop(fixture);
    drop(_guard);
    drop(rt);

    conclusion.exit();
}
