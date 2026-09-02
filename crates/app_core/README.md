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
- [`directory`](src/directory.rs) — the file tree: `list_entries`, `create_directory`,
  `update_directory` (rename and move), `delete_directory`. Also owns `validate_name`,
  shared with `file`.
- [`file`](src/file.rs) — `upload_file`, `download_file`, `download_thumbnail`,
  `update_file`, `delete_file`. Holds the compress-then-encrypt pipeline described below.
- [`sharing`](src/sharing.rs) — granting, listing and revoking permissions on files and
  directories.
- [`access`](src/access.rs) — the effective-access rules the three modules above enforce.
- [`encryption`](src/encryption.rs) — AES-256-GCM envelope encryption.
- [`error::CoreError`](src/error.rs) — the crate-wide error enum. See the error convention
  in [crates/README.md](../README.md).
- [`models`](src/models.rs) — conversions from [database](../database) row structs to the
  shared [models](../models) types (e.g. `api_key_from_db`).

## Stored files

Uploads are **compressed first, encrypted second** — ciphertext does not compress, so the
other order gains nothing:

```txt
upload:   bytes -> image compression -> gzip -> AES-256-GCM -> storage
download: storage -> AES-256-GCM -> gunzip -> bytes
```

Each file carries its own data encryption key, itself encrypted under
`config.storage.encryption_key`. Only the wrapped key reaches [database](../database), and
the object store sees neither the file name nor its type: keys are `files/<uuid>/content`
and `files/<uuid>/thumbnail`.

[storage](../storage) applies compression of its own, so every call from here passes
`CompressionParameters::default()`, which it treats as a byte-for-byte passthrough.

## Access rules

Effective level = the highest of: owning the resource (always `Manager`), an explicit grant
on it, and a grant on — or ownership of — **any ancestor directory**. Levels are ordered
`Viewer < Editor < Manager`, so `access::require` is a `>=` check. A caller that fails it is
answered `CoreError::NotFound`, never a distinct "forbidden", so it cannot probe for what
exists.

## Directory

```txt
app_core/
├── src/
│   ├── api_key.rs      # API key issuing and listing
│   ├── user.rs         # user registration and profile updates
│   ├── directory.rs    # the directory tree
│   ├── file.rs         # upload/download pipeline and file operations
│   ├── sharing.rs      # grants and revocations
│   ├── access.rs       # effective access level of a user on a resource
│   ├── encryption.rs   # AES-256-GCM envelope encryption
│   ├── models.rs       # database row -> shared model conversions
│   ├── error.rs        # CoreError
│   └── lib.rs
└── Cargo.toml
```

## Skills

- [backend-add-api-endpoint](../../.claude/skills/backend-add-api-endpoint/SKILL.md)
