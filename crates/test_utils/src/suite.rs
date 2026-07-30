use std::sync::Arc;

use libtest_mimic::Trial;
use tokio::runtime::Runtime;

/// What a `harness = false` test binary needs from its fixture.
///
/// [`start`](TestSuite::start) brings the environment up — usually a container — and
/// [`trials`](TestSuite::trials) says which suites run against which subjects. That
/// second half stays hand-written on purpose: whether a backend gets a fresh subject
/// per trial or shares one, and which suites apply, is the interesting part of a test
/// binary and belongs where it can be read.
///
/// ```ignore
/// impl TestSuite for GarageFixture {
///     async fn start() -> Self { /* start the container, provision it */ }
///
///     fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
///         super::storage::suite::trials(rt, move || {
///             let fixture = self.clone();
///             async move { S3::try_new(&fixture.endpoint, /* … */).unwrap() }
///         })
///     }
/// }
/// ```
///
/// `start` is a native `async fn` rather than an `#[async_trait]` one on purpose: the
/// generated `main` awaits it once, via `block_on`, on the thread it was called from.
/// Boxing it would demand a `Send` future for no gain, and some container APIs — the
/// `ExecCommand` output reader among them — are not `Send`.
#[allow(async_fn_in_trait)]
pub trait TestSuite: Sized + Send + Sync + 'static {
    /// Brings the environment the suites need up.
    async fn start() -> Self;

    /// Collects the trials to run against this fixture.
    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial>;
}
