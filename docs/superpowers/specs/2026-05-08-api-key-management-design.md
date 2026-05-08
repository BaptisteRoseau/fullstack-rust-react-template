# API Key Management — Design Spec

**Date:** 2026-05-08
**Branch:** `api_key`

---

## Overview

Add full API key management to the application: database schema, database trait methods, HTTP endpoints, authenticator integration, and tests. API keys are hashed with SHA-256 before storage; the raw key is never persisted and is returned only once at creation time.

---

## 1. Database Schema

File: `crates/database/migrations/0003_apikey_table.up.sql`

```sql
CREATE TABLE IF NOT EXISTS api_key (
    id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
    hash VARCHAR(256) UNIQUE NOT NULL,
    name VARCHAR(256) NOT NULL,
    owner UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permissions JSON NOT NULL DEFAULT '[]',
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS idx__api_key__owner ON api_key (owner);
CREATE INDEX IF NOT EXISTS idx__api_key__hash ON api_key (hash);
```

`created_at` / `updated_at` are added automatically by the trigger from migration 0001.

File: `crates/database/migrations/0003_apikey_table.down.sql`

```sql
DROP TABLE IF EXISTS api_key;
DROP INDEX IF EXISTS idx__api_key__owner;
DROP INDEX IF EXISTS idx__api_key__hash;
```

**Key decisions:**
- `id UUID` follows the project PK convention (every table uses `id`).
- `hash` is `VARCHAR(256) UNIQUE` — the lookup column for authentication, indexed separately.
- `owner` is a proper `UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE`.
- Down migration lists indexes explicitly, following the `0002` convention.

---

## 2. Generated Model & Database Trait

### Generated model (`generated_models.rs`, after re-running `build_database_rust_models.sh`)

The raw database representation — lives only in the `database` crate, never imported by `api`:

```rust
pub struct ApiKey {
    id: uuid::Uuid,
    hash: String,
    name: String,
    owner: uuid::Uuid,
    permissions: serde_json::Value,  // JSON array, e.g. ["UploadFile"]
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
```

### Public accessors (`crates/database/src/models.rs`)

The `Crud` derive macro does not generate field getters — all generated fields are private. An explicit `impl ApiKey` block is added in `models.rs` (same crate, so it can access private fields) exposing only the fields needed externally:

```rust
impl ApiKey {
    pub fn id(&self) -> uuid::Uuid { self.id }
    pub fn owner(&self) -> uuid::Uuid { self.owner }
    pub fn name(&self) -> &str { &self.name }
    pub fn hash(&self) -> &str { &self.hash }
    pub fn permissions(&self) -> &serde_json::Value { &self.permissions }
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.created_at }
}
```

### Domain model (`crates/models/src/api_key.rs`)

The `api` and `app_core` crates use this type, never `database::ApiKey` directly. `models` gains a dependency on `rbac`.

```rust
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub name: String,
    pub owner: uuid::Uuid,
    pub permissions: Vec<rbac::Permissions>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

`TryFrom<database::ApiKey>` is implemented in `app_core` (the only crate that depends on both `database` and `models`), deserializing the `serde_json::Value` permissions into `Vec<rbac::Permissions>`.

### New methods on `Database` trait (`crates/database/src/database.rs`)

```rust
async fn create_api_key(
    &mut self,
    owner: Uuid,
    name: String,
    hash: String,
    permissions: serde_json::Value,  // pre-serialized from Vec<rbac::Permissions> in app_core
) -> Result<database::ApiKey, Box<DatabaseError>>;

async fn delete_api_key(&mut self, id: Uuid) -> Result<bool, Box<DatabaseError>>;

async fn read_api_key_by_hash(&self, hash: &str) -> Result<database::ApiKey, Box<DatabaseError>>;
```

`DatabaseError` gets a new `HashCollision` variant for unique constraint violations on `hash`, so the retry loop in `app_core` can pattern-match cleanly.

### Postgres implementation

Uses raw `sqlx::query_as` / `sqlx::query` (same pattern as `read_user` / `delete_user`). On a unique constraint violation for `hash`, returns `DatabaseError::HashCollision`.

---

## 3. Key Generation & Retry (`crates/app_core`)

The raw key is generated and hashed in `app_core`, not in the DB layer. The DB never sees the raw key.

```rust
pub async fn create_api_key(
    db: &mut impl Database,
    owner: Uuid,
    name: String,
    permissions: Vec<rbac::Permissions>,
) -> Result<(String, models::ApiKey), CoreError> {
    let permissions_json = serde_json::to_value(&permissions)?;
    loop {
        let raw_key = generate_random_key(); // 32 random bytes → 64-char hex string
        let hash = hex_sha256(&raw_key);
        match db.create_api_key(owner, name.clone(), hash, permissions_json.clone()).await {
            Ok(db_key) => {
                let api_key = models::ApiKey::try_from(db_key)?;
                return Ok((raw_key, api_key));
            }
            Err(e) if e.is_hash_collision() => continue,
            Err(e) => return Err(e.into()),
        }
    }
}
```

- `generate_random_key`: `rand::random::<[u8; 32]>()` encoded as lowercase hex.
- `hex_sha256`: duplicated as a private function in `app_core` (same 3-line implementation already in `authenticator`). No shared crate dependency is introduced.
- Hash collisions are astronomically rare but handled correctly.

---

## 4. HTTP Endpoints

New directory: `crates/api/src/endpoints/api_key/`

```
api_key/
├── mod.rs
├── models.rs
└── endpoints.rs
```

All endpoints require authentication (`UserToken` extractor — returns 401 if unauthenticated).

### `POST /api-key`

Body: `{ name: String, permissions: Vec<String> }`
Response: `{ id, name, key, permissions, createdAt }` — **only time `key` is returned**.

Calls `app_core::create_api_key` with retry loop. Returns 201.

### `GET /api-key/{id}`

Response: `{ id, name, permissions, createdAt }` — no `key` field.

Loads key by `id`. If not found or `api_key.owner != user.id`, returns 404 (ownership check returns 404, not 403, to avoid leaking existence of other users' keys).

### `DELETE /api-key/{id}`

Returns 204 on success. Same ownership check as GET — returns 404 if not found or not owned by requester.

GET and DELETE call the database trait directly (no `app_core` indirection needed).

---

## 5. Authenticator Integration

File: `crates/authenticator/src/backends/secrets_provider.rs`

Replace the `todo!()` in `validate_api_key`:

```rust
async fn validate_api_key(&self, token: &str) -> Result<UserToken, Box<AuthenticatorError>> {
    let hashed = hex_sha256(token);

    // Check cache first
    if let Some(value) = self.cache.read().await.get_nofail(&hashed).await
        && let Ok(user_token) = serde_json::from_value::<UserToken>(value)
    {
        return Ok(user_token);
    }

    // Look up in DB — returns database::ApiKey, only owner() needed here
    let api_key = self.database.read().await
        .read_api_key_by_hash(&hashed).await
        .map_err(|_| AuthenticatorError::AuthenticationFailure)?;

    let user_token = UserToken {
        id: api_key.owner(),
        realm: "api_key".to_string(),
    };
    // Note: permissions are intentionally not included in UserToken;
    // they are enforced at the endpoint level via the models::ApiKey.

    // Cache result
    // ...

    Ok(user_token)
}
```

- Uses `"api_key"` as realm — stable discriminator for downstream code.
- Any DB error (including not-found) maps to `AuthenticationFailure` — never leaks DB errors to callers.

---

## 6. Tests

### Database trait tests

Structure mirrors `crates/storage/tests/`:

```
crates/database/tests/
├── common/
│   ├── mod.rs
│   ├── containers.rs   — PostgresFixture (testcontainer + sqlx migrations)
│   └── database.rs     — database_trait_tests! macro + assert_* functions
└── postgres.rs         — main() runner using libtest_mimic
```

Test cases in `database_trait_tests!`:
- `assert_create_api_key` — creates a key, verifies all fields
- `assert_read_api_key_by_hash` — creates then looks up by hash
- `assert_delete_api_key` — creates then deletes, verifies gone
- `assert_delete_api_key_nonexistent` — delete on unknown id returns false

### API handler tests (`crates/api/tests/`)

Uses `tower::ServiceExt` with a mock `Database`. Test cases:
- `test_post_api_key` — authenticated POST returns key + metadata
- `test_get_api_key` — authenticated GET returns metadata without `key` field
- `test_get_api_key_wrong_owner` — GET by another user returns 404
- `test_delete_api_key` — authenticated DELETE returns 204
- `test_delete_api_key_wrong_owner` — DELETE by another user returns 404
- `test_unauthenticated_endpoints` — all three endpoints return 401 without Bearer token

### App core unit test (`crates/app_core/`)

- `test_hash_collision_retry` — mock DB fails with `HashCollision` once, succeeds on second call; verifies the loop retries exactly once.
