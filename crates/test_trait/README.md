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
| --- | --- | --- |
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

## Writing a suite

See the [backend-trait-test](../../.claude/skills/backend-trait-test/SKILL.md) skill for
the step-by-step: the suite file, one file per backend, the `Cargo.toml` stanza and the
`tests/README.md` that goes with them.

## Where it is used

| Crate | Trait | Backends run against the suite |
| --- | --- | --- |
| `crates/cache` | `Cache` | `Redis`, `HashMapCache` |
| `crates/database` | `Database` | `Postgres`, `MockDatabase` |
| `crates/storage` | `Storage` | `S3` (Garage) |
| `crates/authenticator` | `Authenticator` | `Keycloak` — two suites, `trials_shared`, a context |

`crates/cache/tests` is the smallest complete example.

## Skills

- [backend-trait-test](../../.claude/skills/backend-trait-test/SKILL.md)
