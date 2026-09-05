# Crates

The Rust backend. Every crate here is a **library**. Binaries live in [binaries](./binaries).

## Layers

Three crates form the main stack. Dependencies flow one way only:

```txt
api → app_core → database
mcp ↗
```

- `api` may use `app_core` and `database`.
- `app_core` may use `database`.
- `database` may use neither.

[mcp](./mcp) is a second protocol crate sitting at the same level as `api`: it may use
`app_core` and `database`, and must never import `api`. `api` mounts its HTTP endpoint, so
the dependency runs `api → mcp` and never the other way.

An inner layer never imports an outer one. This keeps each layer readable and testable on its own.

Every other crate is a **service crate**: it exposes one trait, and the layers above depend on that
trait, never on a concrete backend.

## Directory

```txt
crates/
├── api/                  # HTTP layer: routes, endpoints, extractors, middlewares
├── mcp/                  # Model Context Protocol layer: the tools assistants can call
├── app_core/             # Business logic. The only place domain rules live
├── database/             # Postgres access and migrations
├── models/               # Domain structs shared between layers
├── config/               # CLI and file configuration, read-only once parsed
├── authenticator/        # Identity provider interface (Keycloak backend)
├── cache/                # Key/value cache interface (Redis, in-process map)
├── storage/              # Blob and file interface (S3-compatible backend)
├── compressor/           # Image and blob compression, used by storage
├── rbac/                 # Roles, scopes and permission checks
├── mailer/               # Outgoing email
├── logging/              # Global tracing subscriber setup
├── binaries/             # The only crates allowed a main.rs
├── database_crud_derive/ # Derive macro writing CRUD queries for database models
├── test_trait/           # Runs one test suite against every backend of a trait
├── test_trait_derive/    # Proc macros behind test_trait
└── test_utils/           # Test-only macros shared by every crate
```

## Service crate layout

```txt
<crate>/
├── src/
│   ├── backends/
│   │   ├── mod.rs
│   │   └── <backend>.rs   # one file per implementation
│   ├── <crate>.rs         # the public trait
│   ├── error.rs
│   └── lib.rs
└── tests/
    ├── backends/
    │   └── <backend>.rs   # fixture + test binary, one per backend
    └── trait_tests.rs     # the suite every backend must pass
```

The trait is public and `Send + Sync`. It is the only thing `app_core` and `api` import.
[cache](./cache) is the smallest complete example.

Backends are behind Cargo features, so a build can drop the ones it does not use.

## Conventions

**`lib.rs` and `mod.rs` hold no logic.** Only `mod`, `use` and `pub use`. Real code goes in a new or
existing file.

**Errors** are `thiserror` enums, named after the crate in CamelCase, and live in `error.rs`:

```rust
// crates/my_crate/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum MyCrateError { /* ... */ }
```

**Unit tests** live in a sibling `src/_tests/test_<name>.rs`, not inline. See
[test_utils](./test_utils).

## Tech stack

| Concern | Crate |
| --- | --- |
| HTTP server | Axum, Tower |
| Database and migrations | SQLx |
| CLI parsing | Clap |
| OpenAPI document and Swagger UI | utoipa |
| Errors | thiserror |

## Testing

A test is a **unit test** when it can run alone, and an **integration test** when it needs a real
service such as Postgres, Keycloak or Redis. The question is not "is it slow?" but **"would a mock
make this test tautological?"**.

Integration suites are written once against the trait, then replayed against every backend using
[test_trait](./test_trait) and `testcontainers`.

## Skills

- [backend-add-api-endpoint](../.claude/skills/backend-add-api-endpoint/SKILL.md)
- [backend-add-mcp-tool](../.claude/skills/backend-add-mcp-tool/SKILL.md)
- [backend-config-entry](../.claude/skills/backend-config-entry/SKILL.md)
- [backend-feature-gating](../.claude/skills/backend-feature-gating/SKILL.md)
- [backend-trait-test](../.claude/skills/backend-trait-test/SKILL.md)
- [backend-unit-test-location](../.claude/skills/backend-unit-test-location/SKILL.md)
