# test_trait

Scaffolding for the crates that expose one trait and several backends implementing it.

Those crates test the **trait's contract**, not a backend: the suite is written once and
run against every implementation — `Redis` and `HashMapCache`, `Postgres` and
`MockDatabase`. This crate is what turns such a suite into a `harness = false` test
binary, so that the only thing a backend has to supply is how its environment starts.

It is the crate consumers depend on. The macros themselves live in
[test_trait_derive](../test_trait_derive), re-exported from here so a consumer needs a
single `[dev-dependencies]` entry.

## What it exposes

| Item | Kind | Role |
|---|---|---|
| `TestSuite` | trait | What a test binary needs from its fixture: `start()` and `trials()` |
| `test_trait` | attribute macro | Marks one test inside a suite module |
| `test_trait_suite` | attribute macro | Generates a suite module's trial collectors |
| `test_trait_main!` | function macro | Writes the test binary's `fn main()` |
| `Trial`, `Arguments`, `Conclusion`, `Failed`, `run` | re-export | `libtest_mimic`, named by the generated code |
| `Runtime` | re-export | `tokio::runtime::Runtime`, the one the trials block on |
| `async_trait` | re-export | For the test-side traits a suite sometimes needs |

The re-exports exist so the generated code can name `::test_trait::…` paths without
assuming what the consumer has in scope, and so a consumer adds one dev-dependency
rather than four.

## Usage

### 1. The suite — `tests/trait_tests.rs`

Each test takes the subject as the **trait**, never a backend, and its function name is
the trial name. Unmarked functions are helpers and are left alone.

```rust
use test_trait::{test_trait, test_trait_suite};

#[test_trait_suite]
pub mod suite {
    use super::*;

    #[test_trait]
    async fn set_and_get(cache: &impl Cache) {
        let key = unique_key("set_and_get");
        cache.set(&key, &json!("hello"), None).await.expect("set failed");
        let output = cache.get(&key).await.expect("get failed");
        assert_eq!(output, Some(json!("hello")), "expected Some(\"hello\"), got {output:?}");
    }
}

/// A helper, not a trial: no marker, so the macro ignores it.
fn unique_key(suffix: &str) -> String {
    format!("test:{}:{suffix}", uuid::Uuid::new_v4())
}
```

`#[test_trait_suite]` appends two functions to the module, both returning `Vec<Trial>`:

| Function | Subject | Use it when |
|---|---|---|
| `trials(rt, build)` | freshly built per trial | the default — each test wants a clean backend |
| `trials_shared(rt, subject)` | one `Arc<S>` for every trial | construction is expensive and the backend is stateless |

`trials_shared` only appears when every test takes its subject by shared reference: a
`&mut` or by-value subject cannot come out of an `Arc`.

### 2. The fixture — `tests/backends/<backend>.rs`

`start()` brings the environment up; `trials()` says which suites run against which
subjects. That second half stays hand-written because it is where the real per-backend
decision lives — fresh subject or shared, and which suites apply.

One file per backend, and it is the test binary too: the suite comes in through
`#[path]`, so the fixture, its trials, and `fn main()` all read top to bottom.

```rust
#[path = "../trait_tests.rs"]
mod trait_tests;

test_trait::test_trait_main!(RedisFixture);

struct RedisFixture { /* … */ }

impl TestSuite for RedisFixture {
    async fn start() -> Self { /* start the container */ }

    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        trait_tests::suite::trials(rt, move || {
            let fixture = self.clone();
            async move { RedisBackend::new(&fixture.url, None, None).unwrap() }
        })
    }
}
```

The builder is always async; a backend that constructs synchronously wraps itself in
`async move { … }` so there is one code path through the macro.

`start` is a native `async fn` rather than an `#[async_trait]` one: the generated `main`
awaits it once via `block_on`, and boxing it would demand a `Send` future that some
container APIs cannot provide.

### 3. The targets — `Cargo.toml`

Cargo only auto-discovers `tests/*.rs`, so each backend declares its own stanza:

```toml
[package]
# `tests/trait_tests.rs` is the shared suite, not a target of its own.
autotests = false

[dev-dependencies]
test_trait = { path = "../test_trait" }

[[test]]
name = "redis"                       # what `--test redis` refers to
path = "tests/backends/redis.rs"
harness = false

[[test]]
name = "hash_map"
path = "tests/backends/hash_map.rs"
harness = false
```

A backend that needs no service is just another file in `tests/backends/`, reusing the
same suite for a containerless binary — see `crates/cache/tests/backends/hash_map.rs`.

### Context parameter

When a test needs the provider to do something the trait deliberately does not cover,
it declares a **second parameter** typed against a test-side trait. The generated
collectors then take a matching `context: Arc<C>` and hand it to the tests that asked
for it, so a second backend supplies its own agent and the suite runs unchanged.

```rust
#[test_trait]
async fn exchange_code_returns_tokens(
    authenticator: &impl Authenticator,
    agent: &impl ProviderAgent,
) { /* … */ }
```

`crates/authenticator/tests/trait_tests/provider.rs` is the worked example: it declares
the trait, and `tests/backends/keycloak.rs` implements it for its fixture.

## Where it is used

| Crate | Trait | Backends run against the suite |
|---|---|---|
| `crates/cache` | `Cache` | `Redis`, `HashMapCache` |
| `crates/database` | `Database` | `Postgres`, `MockDatabase` |
| `crates/storage` | `Storage` | `S3` (Garage) |
| `crates/authenticator` | `Authenticator` | `Keycloak` — two suites, `trials_shared`, a context |

`crates/cache/tests` is the smallest complete example. The `backend-trait-test` skill
documents the conventions and the reasons behind them.
