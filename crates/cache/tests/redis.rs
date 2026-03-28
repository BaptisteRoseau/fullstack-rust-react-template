#[macro_use]
mod common;

use std::sync::Arc;

use cache::backends::redis::Redis;
use common::containers::RedisFixture;
use libtest_mimic::Arguments;

fn main() {
    let args = Arguments::from_args();

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let fixture = Arc::new(rt.block_on(async { RedisFixture::start().await }));

    let make_cache = {
        let fixture = fixture.clone();
        move || Redis::new(&fixture.url, None).expect("failed to create Redis client")
    };

    let tests = cache_trait_tests!(make_cache, rt.clone());

    let conclusion = libtest_mimic::run(&args, tests);

    // Drop fixture inside the tokio runtime context so ContainerAsync::Drop
    // can run its async cleanup.
    let _guard = rt.enter();
    drop(fixture);
    drop(_guard);
    drop(rt);

    conclusion.exit();
}
