# Cache

This crate implements the interface with a caching service such as Redis or Valkey.

Two backends implement the `Cache` trait:

- `backends::redis::Redis` — the shared, out-of-process one.
- `backends::hash_map::HashMapCache` — an in-process `HashMap`. Honours `timeout_s`
  the same way `Redis` does, but nothing is shared between processes and nothing
  survives a restart, so it suits single-process deployments and tests that want a
  working cache rather than a stub.

Both are held to the same contract by the trait suite in `tests/common/cache.rs`.
