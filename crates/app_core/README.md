# App Core

The business logic layer. This is the only crate allowed to hold domain rules: given the
data, what is the right thing to do.

See [crates/README.md](../README.md) for the layer rule: `app_core` may use `database`,
but never `api`. Its functions take `&Arc<RwLock<dyn Database>>`, never a concrete backend,
so `api` handlers can call them without knowing which backend is live.

A function locks a service itself, only around each call to it, and releases the lock before
doing anything else. A handler or a tool never locks: holding a lock for a whole request
blocks every other request on the same service, a write lock most of all.

## Public surface

One module per domain concept, each a set of free functions rather than a struct:

- [`user`](src/user.rs) — `register`, `read_profile`, `update_profile` and the
  `create_user` / `update_user` / `get_user` / `delete_user` family.
- [`api_key`](src/api_key.rs) — `list_api_keys`, `create_api_key`, `get_api_key`,
  `delete_api_key`. A raw key is generated, hashed with SHA-256, and only the hash reaches
  [database](../database); `create_api_key` retries key generation on a hash collision.
- [`error::CoreError`](src/error.rs) — the crate-wide error enum. See the error convention
  in [crates/README.md](../README.md).
- [`models`](src/models.rs) — conversions from [database](../database) row structs to the
  shared [models](../models) types (e.g. `api_key_from_db`).

## Access checks

Every function reading or changing stored data on behalf of someone takes the caller as
`&rbac::UserPermissions` and checks access itself, through
[`AccessControl::has_access`](../rbac/src/access_control.rs). The protocol crates, [api](../api)
and [mcp](../mcp), only pass the caller along, so a new entry point cannot forget a check.

A refusal answers `CoreError::NotFound`, the same as a missing object, so a caller cannot
learn that someone else's object exists. The ownership check lives in
[access.rs](src/access.rs).

`register` takes no caller: it writes the identity the provider just verified at login.

## Directory

```txt
app_core/
├── src/
│   ├── access.rs    # the access checks shared by every domain module
│   ├── api_key.rs   # API key issuing, listing, reading and deleting
│   ├── user.rs      # user registration and profile updates
│   ├── models.rs    # database row -> shared model conversions
│   ├── error.rs     # CoreError
│   └── lib.rs
├── tests/           # access checks, run against the in-memory database
└── Cargo.toml
```

## Skills

- [backend-add-api-endpoint](../../.claude/skills/backend-add-api-endpoint/SKILL.md)
- [backend-add-mcp-tool](../../.claude/skills/backend-add-mcp-tool/SKILL.md)
