# App Core

The business logic layer. This is the only crate allowed to hold domain rules: given the
data, what is the right thing to do.

See [crates/README.md](../README.md) for the layer rule: `app_core` may use `database`,
but never `api`. Its functions take `&dyn Database` / `&mut dyn Database`, never a
concrete backend, so `api` handlers can call them without knowing which backend is live.

## Public surface

One module per domain concept, each a set of free functions rather than a struct:

- [`user`](src/user.rs) — `register`, `read_profile`, `update_profile` and the
  `create_user` / `update_user` / `get_user` / `delete_user` family.
- [`api_key`](src/api_key.rs) — `list_api_keys`, `create_api_key`. A raw key is generated,
  hashed with SHA-256, and only the hash reaches [database](../database); `create_api_key`
  retries key generation on a hash collision.
- [`error::CoreError`](src/error.rs) — the crate-wide error enum. See the error convention
  in [crates/README.md](../README.md).
- [`models`](src/models.rs) — conversions from [database](../database) row structs to the
  shared [models](../models) types (e.g. `api_key_from_db`).

## Directory

```txt
app_core/
├── src/
│   ├── api_key.rs   # API key issuing and listing
│   ├── user.rs      # user registration and profile updates
│   ├── models.rs    # database row -> shared model conversions
│   ├── error.rs     # CoreError
│   └── lib.rs
└── Cargo.toml
```

## Skills

- [backend-add-api-endpoint](../../.claude/skills/backend-add-api-endpoint/SKILL.md)
