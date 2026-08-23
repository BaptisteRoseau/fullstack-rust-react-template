# Cache

The key/value cache interface. See [crates/README.md](../README.md) for the service-crate
shape this crate follows.

Two backends implement the [`Cache`](src/cache.rs) trait:

- `backends::redis::Redis` — the shared, out-of-process one.
- `backends::hash_map::HashMapCache` — an in-process `HashMap`. Honours `timeout_s` the
  same way `Redis` does, but nothing is shared between processes and nothing survives a
  restart, so it suits single-process deployments and tests that want a working cache
  rather than a stub.

Values are stored as `serde_json::Value`, so the trait stays object-safe (`dyn Cache`);
callers serialize and deserialize on their side.

## Skills

- [backend-feature-gating](../../.claude/skills/backend-feature-gating/SKILL.md)
- [backend-trait-test](../../.claude/skills/backend-trait-test/SKILL.md)
