# Crates

This folder contains the different crates that make the backend.

Most importantly:

- [api](./api): The API layer, all HTTP endpoints, middlewares and RBAC.
- [app_core](./app_core): The domain layer, where the business logic happens.
- [database](./database): The database layer, exposes traits and API to insterract with the database.
- [config](./config): The configuration either from a file or the CLI, with defaults. This config is passed to all the previous layers and is read-only once parsed.
- [storage](./storage): The API to store blobs and files. Consider your environment read-only, every write operation should either be in the database for data, or the storage API for files.

The module relashionship should be `api` > `app_core` > `database`. Each inner layer cannot import outer layers to keep a coherent architecture and allow working on modules independently.

Here:

- `app_core` cannot use `api`
- `database` can neither use `app_core` nor `api`
- `app_core` can use `database`
- `api` can use `app_core` and `database`

## Types of crates

Each crate is this folder is library only. For binaries, go into [binaries](./binaries) and import the required libraries.

Except `core` and `api`, most crates follow this pattern:

```
mycrate
├── Cargo.toml
├── README.md
├── src
│   ├── backends
│   │   ├── mod.rs
│   │   └── some_backend.rs
│   ├── mycrate.rs
│   ├── error.rs
│   └── lib.rs
└── tests
    ├── common
    │   ├── cache.rs
    │   ├── containers.rs
    │   └── mod.rs
    └── some_backend.rs
```

The crate exposes a public trait that is `Send + Sync`, this is the one that will be used in `app_core` and `api` crates.
In `src/backends` are store structs that implement this trait.

## mods.rs and lib.rs

These file should never contain custom code, but only `mod` and `use` imports and export.

If you need to add custom code to them, put it in a new or existing file instead.

## Tech Stack

The tech stack used in the backend is:

- Axum and Tower for the API layer
- SQLx for the database layer and migrations
- Clap for CLI interface
- utoipa for the openapi.json and swagger UI

## Testing

Tests are split into unit tests and integration tests. Unit tests are standalone tests on small pieces of code, whereas integration tests expect to interact with an environment like a database or an API.

The dividing question is not "is it slow?" but **"would a mock make this test tautological?"** If the behaviour under test lives in Postgres or Keycloak, the test needs Postgres or Keycloak. Those use the `testcontainers` library and run in parallel against one shared container.

An integration test suite is written once **against the trait** and run against every backend implementing it. `crates/test_utils` provides the scaffolding:

- `#[trait_test_suite]` on a module of `#[trait_test]` functions generates the trial collector, so each test is named only once — in its own signature.
- `trait_test_main!(MyFixture)` writes the `harness = false` binary's `fn main()`.
- Fixtures implement `test_utils::TestSuite` to say how the environment starts and which suites run against which subjects.

`crates/cache/tests` is the smallest complete example. The `backend-trait-test` skill documents the conventions and the reasons behind them.

## Errors

Errors should be derived from `thiserror`, name `CamelCaseModuleError` and reside in the `error` file.

For example, the errors for my_crate should be named `MyCrateError` and be located under `my_crate/src/error.rs`
