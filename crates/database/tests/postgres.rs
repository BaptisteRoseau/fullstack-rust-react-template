#[macro_use]
mod common;

use std::sync::Arc;

use common::containers::PostgresFixture;
use libtest_mimic::Arguments;

fn main() {
    let args = Arguments::from_args();

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let fixture = Arc::new(rt.block_on(PostgresFixture::start()));

    let make_db = {
        let fixture = fixture.clone();
        move || {
            let fixture = fixture.clone();
            async move { fixture.make_postgres().await }
        }
    };

    let tests = database_trait_tests!(make_db, rt.clone());
    let conclusion = libtest_mimic::run(&args, tests);

    // Drop fixture inside the tokio runtime context so ContainerAsync::Drop
    // can run its async cleanup.
    let _guard = rt.enter();
    drop(fixture);
    drop(_guard);
    drop(rt);

    conclusion.exit();
}
