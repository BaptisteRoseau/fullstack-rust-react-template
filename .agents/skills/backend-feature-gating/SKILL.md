---
name: backend-feature-gating
description: Load this skill BEFORE adding a new backend under a trait crate's `src/backends/` (redis, postgres, s3, keycloak, or a new one), before creating a new trait crate that follows that same `src/backends/` pattern, and before editing any crate's `[features]` table in `Cargo.toml` or its `src/backends/mod.rs`. Also load it when reviewing such a change, since a bad gate silently breaks the non-feature build without failing `--all-features` CI.
---

# Feature-Gating Backend Implementations

Every backend under a trait crate's `src/backends/` (see `crates/README.md`, "Types of crates") is gated behind a disabled-by-default Cargo feature, so a consumer that only needs the trait doesn't pull the backend's dependencies (redis client, sqlx, aws-sdk, ...).

## Checklist for a new backend

- [ ] Add a feature in the crate's `Cargo.toml` `[features]`, named after the backend module (e.g. `postgres = []`).
- [ ] Gate **both** the module and its re-export in `src/backends/mod.rs`:
  ```rust
  #[cfg(feature = "postgres")]
  mod postgres;
  #[cfg(feature = "postgres")]
  pub use postgres::Postgres;
  ```
- [ ] Add `required-features = ["postgres"]` to the matching `[[test]]` stanza for its integration-test binary.
- [ ] In any crate that constructs the concrete type directly (not just the trait object) — e.g. `crates/binaries/backend` — enable the feature on that path dependency.
- [ ] If another crate's tests use the backend as a double (e.g. `HashMapCache` standing in for `Cache` in `authenticator`'s tests), enable the feature on that crate's `[dev-dependencies]` entry, not `[dependencies]`.
- [ ] Verify **both** build configurations, not just the one CI runs:
  - `cargo clippy -p <crate> --all-features` (what `test_lint.sh` runs)
  - `cargo check -p <crate> --no-default-features` (what a trait-only consumer gets — CI never runs this, so it's the only way to catch dead code left behind in an ungated shared file, e.g. an error-constructor only ever called from the now-gated backend)

## Do / Don't

| Do | Don't |
|---|---|
| Gate both the `mod` and its `pub use` | Gate only one of the two, leaving a dangling re-export or an unreachable module |
| Name the feature after the backend module | Invent an unrelated feature name |
| Enable the feature only on the specific dependency edge that needs it (consumer binary, dev-dependency) | Turn the feature on by default inside the trait crate itself |
| Run the `--no-default-features` check after gating | Trust `--all-features` CI alone |
| Add `required-features` to the integration test's `[[test]]` stanza | Leave the integration test buildable without its backend feature |

## Rationale

`doc/backend-feature-gating.md` has the narrative version of this pattern — keep both in sync if it changes.
