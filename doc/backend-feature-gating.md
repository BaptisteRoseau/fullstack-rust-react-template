# Reminder — feature-gate backend crate implementations

Every trait crate under `crates/` that follows the `src/backends/` pattern (see
[`crates/README.md`](../crates/README.md), "Types of crates") exposes its concrete backends
behind Cargo features, disabled by default. `cache` (`redis`, `hash_map`), `database`
(`postgres`), `storage` (`s3`) and `authenticator` (`keycloak`) all follow this.

When adding a **new backend** to one of these crates, or a **new trait crate** with its own
`src/backends/`, remember to:

1. Add a feature for it in the crate's `Cargo.toml` under `[features]` (name matches the
   backend module), and gate its `mod`/`pub use` in `src/backends/mod.rs` behind
   `#[cfg(feature = "...")]`.
2. Add `required-features = ["..."]` to the matching `[[test]]` stanza, so the backend's
   trait-suite binary only builds when the feature is enabled.
3. In any crate that consumes the concrete backend type directly (not just the trait object),
   enable the feature on that path dependency — e.g. `crates/binaries/backend/Cargo.toml` picks
   `postgres`, `redis`, `s3` and `keycloak` because `program.rs` constructs those concrete types.
4. If a backend is also used from another crate's tests or `#[cfg(test)]` code (e.g. the
   `cache` crate's `hash_map` backend standing in for a real cache in `authenticator`'s tests),
   enable that feature on the `[dev-dependencies]` entry, not `[dependencies]`.

`scripts/test_units.sh` and `scripts/test_lint.sh` already pass `--all-features`, so a missed
gate here surfaces as a broken default (non-features) build, not a missing test run.
