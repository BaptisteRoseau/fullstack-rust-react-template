---
name: backend-feature-gating
description: Use when adding a backend under a trait crate's src/backends/, creating a new trait crate, or editing a [features] table.
---

# Feature-gate a backend implementation

Every backend under a trait crate's `src/backends/` sits behind a Cargo feature that is **off by
default**. A crate that only needs the trait must not pull in a Redis client, sqlx or an AWS SDK.

See [crates/README.md](../../../crates/README.md) for the service-crate layout.

A wrong gate does not fail CI. `test_lint.sh` and `test_units.sh` both pass `--all-features`, so the
build that breaks is the one nobody runs.

## 1. Declare the feature

In the crate's `Cargo.toml`, add a feature named exactly after the backend module:

```toml
[features]
# Each backend under `src/backends` is opt-in; enable the ones you need.
redis = []
hash_map = []
```

Never enable it by default inside the trait crate.

## 2. Gate the module and its export

In `src/backends/mod.rs`, put `#[cfg(feature = "...")]` on **every** line the backend owns. Gating
the module but not its re-export leaves a dangling export that fails to compile.

Two shapes are in use. Prefer the first: it keeps the module private and exports only the type.

```rust
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "postgres")]
pub use postgres::Postgres;
```

```rust
#[cfg(feature = "hash_map")]
pub mod hash_map;
```

## 3. Gate the test binary

Each backend has its own test binary. Add `required-features` so it is only built when its backend
is enabled:

```toml
[[test]]
name = "redis"
path = "tests/backends/redis.rs"
harness = false
required-features = ["redis"]
```

See Skill(backend-trait-test) for what goes inside that binary.

## 4. Enable the feature on the consumers

A crate that names the **concrete type** must turn the feature on for that dependency. A crate that
only uses the trait must not.

- Constructs the type in normal code, such as
  [crates/binaries/backend](../../../crates/binaries/backend): enable it under `[dependencies]`.
- Uses the backend only as a test double, such as `HashMapCache` standing in for a real cache in
  another crate's tests: enable it under `[dev-dependencies]`, never `[dependencies]`.

## Checklist

```bash
./scripts/test_no_default_features.sh    # the build CI never runs
cargo clippy --workspace --all-features -- -A clippy::module_inception
```

`test_no_default_features.sh` is the one that matters. It compiles the workspace with every optional
feature off, which is how you find code left unreachable behind a new gate — for example an error
constructor only ever called from the backend you just gated.

- [ ] The feature name matches the module name.
- [ ] The trait crate still builds with no features at all.
