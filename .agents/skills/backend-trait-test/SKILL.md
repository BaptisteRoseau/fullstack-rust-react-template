---
name: backend-trait-test
description: Load this skill when adding or changing a trait or backend in `crates/`, writing or fixing their tests, adding a test double or fixture, or deciding between a unit test and a container test.
---

# Skill: Test a Backend Trait

Most crates in `crates/` follow one shape: **one public trait, one or more backends implementing it**. The trait is what `app_core` and `api` depend on; the backend is an implementation detail. Tests follow from that shape — you test the *trait's contract* once, then run that same suite against every backend.

This pays off the moment a second backend appears: the suite is the specification, and a new backend either satisfies it or does not.

---

## 1. Crate layout

```
mycrate
├── Cargo.toml
├── README.md
├── src
│   ├── lib.rs            # only `mod` / `pub use` — no code
│   ├── mycrate.rs        # THE trait, named after the crate
│   ├── models.rs         # types the trait's signatures use
│   ├── error.rs          # MyCrateError (thiserror)
│   └── backends
│       ├── mod.rs        # mod some_backend; pub use some_backend::SomeBackend;
│       └── some_backend.rs
└── tests
    ├── README.md
    ├── assets/           # fixtures the tests need, owned by THIS crate
    └── common
        ├── mod.rs        # the test binary's entry point
        ├── containers.rs # the testcontainers fixture
        └── mycrate.rs    # the trait test suite
```

The trait is `Send + Sync` so it can live in `AppState` as `Arc<RwLock<dyn MyTrait>>`. Keep methods `&self` unless a method genuinely mutates — a single `&mut self` method forces every caller through a write lock.

When a backend grows past ~200 lines, promote `backends/some_backend.rs` to a directory with a `mod.rs` that only re-exports, and split by responsibility. `crates/authenticator/src/backends/keycloak/` does this: `backend.rs` (struct + trait impl) delegating to `endpoints.rs`, `jwt.rs`, `api_key.rs`, `oidc.rs`.

---

## 2. Unit test or integration test?

Ask what the test needs to be true.

**Unit test — `#[cfg(test)] mod tests` in the same file.** Pure functions, parsing, key derivation, anything you can drive with an in-memory double. These must stay instant; the whole workspace's unit tests run in well under a second.

**Integration test — `tests/`, real service in a container.** Anything whose answer depends on the service actually behaving: SQL constraints, S3 signatures, TTL semantics, a real OAuth flow.

The dividing question is not "is it slow?" but **"would a mock make this test tautological?"** A test asserting that a mock returns what you told it to return proves nothing. If the interesting behaviour lives in Postgres or Keycloak, the test needs Postgres or Keycloak.

```rust
// Unit test — the digest format is ours, no service can tell us if it's right.
#[test]
fn hex_sha256_matches_known_vectors() {
    let empty = hex_sha256("");
    assert_eq!(
        empty, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "sha256 of empty string mismatch, got={empty}"
    );
}
```

Every assertion carries a message interpolating the actual values. A bare `assert!(result.is_ok())` that fires in CI tells you nothing; `got={result:?}` tells you everything.

---

## 3. Shared test doubles

Unit tests need in-memory implementations of the *other* crates' traits. Do not hand-roll one per test module — that is how a repo ends up with three subtly different `MockCache`s.

First ask whether the in-memory version is a *double* at all. `cache::backends::hash_map::HashMapCache` started as one, but a `HashMap` behind a mutex is a perfectly good single-process cache, so it is an ordinary backend: no feature gate, no `testing` module, and downstream tests just use it. Prefer that whenever the in-memory implementation is something you would ship.

What is left over — an implementation that only makes sense in a test, because parts of it are `unimplemented!()` or rigged to fail — ships behind a `test-utils` feature:

```toml
# crates/database/Cargo.toml
[features]
# In-memory `Database` test double (`database::testing::MockDatabase`), for
# downstream crates' tests. Enable via [dev-dependencies].
test-utils = []
```

```rust
// crates/database/src/lib.rs
#[cfg(feature = "test-utils")]
pub mod testing;
```

Consumers opt in from `[dev-dependencies]` only, so it never reaches a release build:

```toml
[dev-dependencies]
database = { path = "../database", features = ["test-utils"] }
config   = { path = "../config",   features = ["test-utils"] }
```

Available today: `database::testing::MockDatabase` and `config::testing::test_config()` behind the feature, `cache::backends::hash_map::HashMapCache` as a plain backend. Prefer extending one of these over writing a local stub.

Either way the in-memory implementation runs the trait suite like any other backend — see §7. For a gated one that means the crate's *own* test binary needs the feature too, and an integration test links the crate the way any consumer does. Say so with `required-features` rather than a self dev-dependency (`database = { path = "." }`) — the self-dependency works but changes the graph shape, and in this workspace it has repeatedly left stale artifacts that fail the next build with `colliding StableCrateId` or `crate X required to be available in rlib format`:

```toml
[[test]]
name = "mock"
path = "tests/mock.rs"
harness = false
required-features = ["test-utils"]
```

The cost is that a bare `cargo test -p database` silently skips that binary. `scripts/test_units.sh` and `scripts/test_lint.sh` pass `--all-features` for exactly this reason; run them, or `cargo test -p database --all-features`, before believing a double is green.

`config::testing::test_config()` matters more than it looks: it returns a fully populated `Config` with inert values, so a test overrides only the two or three fields it cares about. Hand-rolled `Config` literals have to be edited every time the struct gains a field.

```rust
fn config(&self, realm: &str) -> Config {
    let mut config = config::testing::test_config();
    config.authenticator.issuer_url = self.issuer_url(realm);
    config.authenticator.audiences = vec![AUDIENCE.to_string()];
    config
}
```

---

## 4. The container fixture — `tests/common/containers.rs`

One struct that starts the service and exposes what tests need to reach it.

```rust
use testcontainers::core::ContainerPort::Tcp;
use testcontainers::core::WaitFor;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, runners::AsyncRunner};

const IMAGE: &str = "quay.io/keycloak/keycloak";
/// Pinned: the tests read this service's HTML, so an unannounced upgrade must
/// not silently change what they drive.
const TAG: &str = "26.6.4";
const PORT: u16 = 8080;

/// Imported on startup. `include_str!` a fixture owned by THIS crate.
const REALM_EXPORT: &str = include_str!("../assets/realm-export.json");

pub struct KeycloakFixture {
    #[allow(dead_code)] // held only so Drop tears the container down
    container: ContainerAsync<GenericImage>,
    pub base_url: String,
}

impl KeycloakFixture {
    pub async fn start() -> Self {
        let container = GenericImage::new(IMAGE, TAG)
            .with_exposed_port(Tcp(PORT))
            .with_wait_for(WaitFor::message_on_stdout("started in"))
            .with_cmd(["start-dev", "--import-realm"])
            .with_env_var("KC_BOOTSTRAP_ADMIN_USERNAME", "admin")
            .with_copy_to(
                "/opt/keycloak/data/import/realm-export.json",
                REALM_EXPORT.as_bytes().to_vec(),
            )
            .start()
            .await
            .expect("failed to start keycloak container");

        let port = container
            .get_host_port_ipv4(PORT)
            .await
            .expect("failed to get keycloak http port");

        Self { container, base_url: format!("http://127.0.0.1:{port}") }
    }
}
```

When `testcontainers-modules` has the service, use it instead of `GenericImage` — it knows the ports and the readiness signal:

```rust
use testcontainers_modules::postgres::Postgres as PgImage;

let container = PgImage::default()
    .with_init_sql(UUIDV7_INIT_SQL.as_bytes().to_vec())
    .start().await.expect("failed to start postgres container");
let port = container.get_host_port_ipv4(5432).await.unwrap();
```

The fixture also builds the backend under test, wiring in the shared doubles for whatever else it depends on:

```rust
/// The cache is a working in-memory one because the login flow round-trips its
/// state through it; the database only has to report "not found".
pub async fn authenticator(&self) -> Keycloak {
    let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(HashMapCache::default()));
    let database: Arc<RwLock<dyn Database>> = Arc::new(RwLock::new(MockDatabase::default()));
    Keycloak::try_new(&self.config(), cache, database)
        .await
        .expect("failed to build the keycloak authenticator")
}
```

Say *why* each double is the shape it is. A no-op cache that returns `None` from `get` will make any state round-trip fail, and the next person needs to know that was a choice.

**Fixtures belong to the crate that reads them.** `include_str!` a path inside your own `tests/assets/`, never one reaching into `infrastructure/` or a sibling crate. A deployment file that gets deleted takes the whole workspace build down with it, and a fixture two crates away goes stale silently.

---

## 5. The trait suite — `tests/common/<trait>.rs`

A suite is a module of `async fn`s typed against the **trait**, marked `#[test_trait]`, inside a module marked `#[test_trait_suite]`. The module attribute reads the markers and generates the collector, so **each test is named in exactly one place** — its own signature.

```rust
use test_trait::{test_trait, test_trait_suite};

/// Integration tests for the Storage trait, run against every backend.
///
/// When adding a test here:
/// - mark it `#[test_trait]` and take the subject as `&impl Storage`; the function
///   name becomes the test name, and that is the only place it is written
/// - helpers are unmarked functions, left alone by the macro
#[test_trait_suite]
pub mod suite {
    use super::*;

    #[test_trait]
    async fn save_overwrite(storage: &impl Storage) {
        let path = unique_path();
        let params = no_compression();

        storage.save(&path, b"version-1", &params).await.expect("first save failed");
        storage.save(&path, b"version-2", &params).await.expect("second save failed");

        let loaded = storage.load(&path).await.expect("load failed");
        assert_eq!(loaded, b"version-2", "the second save should win, got={loaded:?}");

        let _ = storage.delete(&path).await;
    }
}

/// Generate a unique test path to avoid blob collisions between parallel tests.
fn unique_path() -> PathBuf {
    PathBuf::from(format!("test-trait/{}", Uuid::new_v4()))
}
```

### The subject is always the trait, never a backend

**A `#[test_trait]` must never name a concrete type.** Write `&impl Storage`, not `&S3`; `&mut impl Database`, not `mut db: Postgres`. `dyn Trait` and a generic parameter of the test's own (`fn t<D: Database>(db: &mut D)`) are equally fine — what matters is that the signature mentions the trait and nothing below it.

This is worth being absolute about because breaking it costs nothing up front and everything later. A concrete subject compiles, passes, and reads almost identically; the suite just quietly stops being a trait suite. Nothing tells you until a second backend arrives and the "reusable" suite turns out to need rewriting — which is exactly what happened to `crates/database`, whose suite took `mut db: Postgres` and reached through `db.pool()` into raw SQL. It never needed to: `Database::register` already created the user its helper wanted.

So when a test seems to need something the trait cannot express, the answer is almost never a concrete type. Either the trait is missing a method the production code also wants, or the need belongs behind a test-side trait the fixture implements (§6), or it is not a trait test at all and belongs in a unit test next to the backend.

Because the suite only knows the trait, a backend needing no service at all reuses it unchanged — `crates/cache/tests/hash_map.rs` and `crates/database/tests/mock.rs` are second `[[test]]` binaries whose fixture starts nothing, `#[path]`-include the same suite file, and call the same generated `trials`. Every in-memory implementation gets one, doubles included: a double nobody holds to the contract is a double whose users are testing behaviour no real backend has.

All trials share one container and run in parallel, so **every test derives its own keys** — `Uuid::new_v4()` in the path, the bucket key, the username, the email. Two tests sharing a row or a blob will pass alone and fail together.

### What the marker is for

`#[test_trait]` distinguishes tests from **async helpers**. `crates/authenticator/tests/common/oidc.rs` has an `async fn log_in(…)` that every flow test calls; without a marker the macro would have to guess, and a naming convention would do it less explicitly. A marker outside a suite module is a compile error rather than a test that quietly never runs.

### What gets generated

Two entry points, both returning `Vec<Trial>`:

| Function | Subject | Use it when |
|---|---|---|
| `trials(rt, build)` | freshly built per trial | the default — each test wants a clean backend |
| `trials_shared(rt, subject)` | one `Arc`, all trials | construction is expensive and the backend is stateless |

`trials_shared` is only generated when every test takes its subject by shared reference, since `&mut` and by-value cannot come out of an `Arc`. `authenticator` uses it because building the backend re-fetches the JWKS and its cache entries are keyed by a random CSRF state, so parallel trials cannot collide.

The macro reads each test's first parameter and emits the matching call, so all these work without configuration:

| First parameter | Generated call |
|---|---|
| `s: &impl Storage` | `f(&subject)` |
| `db: &mut impl Database` | `f(&mut subject)` |
| `db: impl Database` (owned) | `f(subject)` |
| any of the above under `trials_shared` | `f(&*subject)` |

Every test in a suite must agree on the subject type — one suite drives one backend, and the macro says so if they diverge.

`tests/common/mod.rs` declares the modules and is itself the binary's entry point — see the next section.

---

## 6. When the trait can't express what the test needs

Sometimes a test needs the service to do something the trait deliberately doesn't cover. Exchanging an OAuth code needs a real authorization code, which only a browser can get.

Don't reach for the concrete fixture inside the assertions — that couples the suite to one backend again. Put the provider-specific work behind a small **test-side trait**, and let the fixture implement it:

```rust
/// The provider-side actor the trait suite drives. Implemented per provider so
/// the trait tests stay backend-agnostic.
#[async_trait]
pub trait ProviderAgent {
    /// Acts as the end user at the provider's login page and returns the `code`
    /// and `state` the provider redirects back with.
    async fn login(&self, authorize_url: &str) -> CallbackParams;

    /// A freshly issued access token for the credentials realm.
    async fn issue_token(&self) -> String;
}

#[test_trait]
async fn exchange_code_returns_tokens(
    authenticator: &impl Authenticator,
    agent: &impl ProviderAgent,
) { /* ... */ }
```

A test may declare this as a **second parameter**; when any test in a module does, the generated entry points take a matching `context: Arc<C>` and hand it to the tests that asked for it. A second backend supplies its own agent and the whole suite runs against it unchanged. See `crates/authenticator/tests/common/provider.rs`.

---

## 7. The test binary — `tests/common/mod.rs`

`harness = false` means this is a real `fn main()`, not `#[test]` functions. That is what lets one container serve every trial, and `test_trait_main!` writes it. Since that leaves nothing else to say, the module file that declares the suite *is* the binary — there is no separate `tests/<backend>.rs` holding a single line:

```rust
mod containers;
mod storage;

test_trait::test_trait_main!(containers::GarageFixture);
```

Cargo only auto-discovers `tests/*.rs`, so the target is declared explicitly with a `path` (see the manifest below). The target name is what `--test` takes: `cargo test -p storage --test s3`.

It expands to the runtime, the fixture startup, `libtest_mimic::run`, and the teardown — including dropping the fixture inside `rt.enter()`. That drop is not ceremony: `ContainerAsync::Drop` spawns async cleanup, and outside the runtime that cleanup silently cannot run, leaking containers onto the developer's machine. It lives in the macro now so nobody has to remember it.

The fixture supplies the two things the macro cannot know, via `test_trait::TestSuite`:

```rust
#[async_trait]
impl TestSuite for GarageFixture {
    async fn start() -> Self {
        let fixture = Self::start_container().await;
        fixture.create_bucket(TEST_BUCKET).await;
        fixture
    }

    /// A fresh client per trial: connecting is cheap and the suite namespaces its
    /// own paths, so there is nothing to gain from sharing one.
    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        super::storage::suite::trials(rt, move || {
            let fixture = self.clone();
            async move {
                S3::try_new(&fixture.endpoint, TEST_BUCKET, &fixture.access_key, &fixture.secret_key)
                    .expect("failed to create S3 client")
            }
        })
    }
}
```

`trials()` stays hand-written because that is where the real per-backend decisions live — builder versus shared subject, and which suites run. `authenticator` shows why generating it would be a straitjacket: it runs two suite modules against two differently-configured authenticators and concatenates them.

The builder is always async; a backend that constructs synchronously wraps it in `async move { … }`, which keeps one code path through the macro.

Register the binary in `Cargo.toml`, one stanza per backend:

```toml
[dev-dependencies]
test_trait = { path = "../test_trait" }
testcontainers = "0.24"
testcontainers-modules = { version = "0.12", features = ["redis"] }  # if applicable
uuid = { version = "1", features = ["v4"] }

[[test]]
name = "s3"                     # what `--test s3` refers to
path = "tests/common/mod.rs"    # not auto-discovered, so say where it is
harness = false
```

Finally, write `tests/README.md`. Keep it to what a newcomer cannot infer from the code: which fixture is available, what the suite covers, any fixture that has to be shaped a particular way, and the command to run it. `crates/storage/tests/README.md` is the short form; `crates/authenticator/tests/README.md` is the long form, because that suite scrapes a login page and the next person needs to know which details are load-bearing.

---

## 8. Living examples

Read the closest one before writing a new suite.

| Crate | Trait | Backend | Container | Notable for |
|---|---|---|---|---|
| `crates/cache` | `Cache` | `Redis`, `HashMapCache` | `testcontainers-modules` redis | The smallest complete example — start here; two binaries, one containerless |
| `crates/database` | `Database` | `Postgres`, `MockDatabase` | `testcontainers-modules` postgres, migrations run in the fixture | A `&mut impl` subject; schema-dependent tests; the double held to the same contract |
| `crates/storage` | `Storage` | `S3` | Garage via `GenericImage`, provisioned with `ExecCommand` | Post-start provisioning inside `start()`, binary fixtures |
| `crates/authenticator` | `Authenticator` | `Keycloak` | Keycloak via `GenericImage`, two realms imported | `trials_shared`, two suite modules, a context parameter, the `ProviderAgent` pattern |

The macros themselves live in `crates/test_trait` (the `TestSuite` trait and re-exports) and `crates/test_trait_derive` (the proc-macro). Their `tests/fixtures/*.rs` pin the error message for every way of writing a suite that would collect nothing.

---

## 9. Verify

```bash
cargo fmt -p mycrate
cargo clippy -p mycrate --all-targets --all-features
cargo test -p mycrate --lib                  # unit tests, must be instant
cargo test -p mycrate --all-features         # + every suite; needs Docker running
```

`--all-features` is not optional: without it a binary carrying `required-features` is skipped, and cargo says nothing about it.

Filter a single trial by name while iterating — libtest-mimic accepts the same arguments as the normal harness:

```bash
cargo test -p authenticator --test keycloak -- exchange_code --nocapture
```

A whole container suite should land in seconds once the image is pulled; the authenticator's 13 trials take ~0.8s against a container that boots in ~9s. If a suite is much slower than that, something is being rebuilt per test that should be shared.
