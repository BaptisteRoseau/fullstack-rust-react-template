---
name: backend-trait-test
description: Use when adding or changing a trait, a backend or a test double in crates/, or when deciding between a unit test and a container test.
---

# Test a Backend Trait

## 1. What this is

Most crates in `crates/` expose **one trait** and **one or more backends** implementing it. Test the trait's contract once, then run that same suite against every backend. A new backend either satisfies it or does not.

**Apply this skill when** the crate has a trait in `src/<mycrate>.rs` and implementations in `src/backends/` (`cache`, `database`, `storage`, `authenticator`). **Skip it** for crates with no backends (`api`, `app_core`, `config`, `models`, the `*_derive` proc-macros).

```txt
crates/mycrate
├── Cargo.toml               # autotests = false + one [[test]] per backend file
├── src
│   ├── mycrate.rs           # THE trait
│   ├── testing.rs           # optional in-memory double, behind `test-utils`
│   └── backends
│       ├── mod.rs
│       └── some_backend.rs
└── tests
    ├── README.md
    ├── assets/              # files the fixtures copy into containers
    ├── backends
    │   ├── some_backend.rs  # fixture + fn main(); ONE [[test]] binary
    │   └── mock.rs          # the double, written exactly like a backend
    └── trait_tests.rs       # the suite, shared by every backend
```

One binary per backend, all running the same suite. Each `tests/backends/*.rs` is a crate root, so it reaches the suite with `#[path = "../trait_tests.rs"] mod trait_tests;`.

Read the closest existing example before writing a new one. [`crates/cache/tests`](../../../crates/cache/tests) is the smallest complete one.

| Crate | Trait | Backends | Notable for |
| --- | --- | --- | --- |
| [`cache`](../../../crates/cache/tests) | `Cache` | [`redis.rs`](../../../crates/cache/tests/backends/redis.rs), [`hash_map.rs`](../../../crates/cache/tests/backends/hash_map.rs) | Start here. Two binaries, one containerless, both on [`trait_tests.rs`](../../../crates/cache/tests/trait_tests.rs) |
| [`database`](../../../crates/database/tests) | `Database` | [`postgres.rs`](../../../crates/database/tests/backends/postgres.rs), [`mock.rs`](../../../crates/database/tests/backends/mock.rs) | `&mut impl` subject; migrations in `start()`; feature-gated double |
| [`storage`](../../../crates/storage/tests) | `Storage` | [`s3.rs`](../../../crates/storage/tests/backends/s3.rs) | `GenericImage`, post-start provisioning, an asset copied into the container |
| [`authenticator`](../../../crates/authenticator/tests) | `Authenticator` | [`keycloak.rs`](../../../crates/authenticator/tests/backends/keycloak.rs) | [Two suite modules](../../../crates/authenticator/tests/trait_tests), `trials_shared`, [`ProviderAgent`](../../../crates/authenticator/tests/trait_tests/provider.rs) context |

The macros live in [`crates/test_trait`](../../../crates/test_trait) (the `TestSuite` trait and re-exports) and [`crates/test_trait_derive`](../../../crates/test_trait_derive) (the proc-macro); their READMEs cover what is generated and every compile error it emits.

## 2. Step by step

Copy-pastable boilerplate sits in `assets/` next to this file.

### Step 1 — the suite: `tests/trait_tests.rs`

Copy `assets/trait_tests.rs`. Then:

- Type every subject as `&impl MyTrait` (or `&mut impl MyTrait`, or owned `impl MyTrait`). Never a concrete backend — the macro rejects it.
- Write one `#[test_trait] async fn` per behaviour. The function name is the test name; write it nowhere else.
- Give every assert a message interpolating the values: `assert_eq!(got, want, "…, got={got:?} want={want:?}")`.
- Derive per-test keys with `Uuid::new_v4()` — trials share one service and run in parallel.
- Leave helpers unmarked; the macro ignores them.

Two variants:

- **The trait spans two roles** (e.g. credentials + login flow): make `trait_tests/` a directory with `mod.rs` declaring `pub mod <role>;`, one `#[test_trait_suite]` per file, and put constants both suites assert on in `mod.rs`. Backends then use `#[path = "../trait_tests/mod.rs"]`. See `crates/authenticator/tests/trait_tests/`.
- **A test needs something the trait cannot express** (act as a browser, mint a credential): declare a test-side trait in `trait_tests/provider.rs`, take it as the test's **second** parameter (`agent: &impl ProviderAgent`), and implement it in the backend file. The generated collectors then take a `context: Arc<C>`.

### Step 2 — one file per backend: `tests/backends/<backend>.rs`

Copy `assets/backend_container.rs` (needs a service) or `assets/backend_in_memory.rs` (does not). Keep this order and put nothing else in the file:

1. `//!` one-line doc — "Runs the MyTrait trait suite against the X backend."
2. `use` declarations
3. `#[path = "../trait_tests.rs"] mod trait_tests;`
4. `test_trait::test_trait_main!(XFixture);`
5. `struct XFixture`
6. `impl TestSuite for XFixture` — `start()` brings the environment up, `trials()` picks builder or `trials_shared`
7. `impl XFixture` — private helpers of the fixture
8. `const`s, then bare `fn` helpers

Keep every item private. Alias the backend to `XBackend` only when the name collides with the container image type.

### Step 3 — the in-memory implementation gets a file too

If the crate has an in-memory implementation, it gets its own `tests/backends/*.rs` and runs the same suite. It is not optional: a double nobody holds to the contract makes every downstream test pass against behaviour no real backend has.

- Shippable in-memory implementation (e.g. a `HashMap` behind a mutex) → a plain backend in `src/backends/`, no feature gate.
- Only usable in tests (`unimplemented!()` parts, rigged failures) → `src/testing.rs` behind a `test-utils` feature, exported for downstream crates, and its `[[test]]` stanza carries `required-features = ["test-utils"]`.

### Step 4 — `Cargo.toml`

Merge `assets/Cargo.toml`: `autotests = false`, one `[[test]]` stanza per file in `tests/backends/` with `path` and `harness = false`.

### Step 5 — `tests/README.md`

A tree of `tests/`, then only what the code cannot say: which fixture exists, what has to be shaped a particular way, and `cargo test -p mycrate`. Short form: `crates/storage/tests/README.md`. Long form: `crates/authenticator/tests/README.md`.

### Step 6 — run the checklist in §4

## 3. Rules that are easy to get wrong

| Do | Don't |
| --- | --- |
| Type subjects as `&impl MyTrait` | Type them as `&Postgres`, or reach through `db.pool()` into raw SQL |
| Name a test once, in its `async fn` signature | Hand-write a `Vec<Trial>` or a `#[test]` fn in these binaries |
| Derive per-trial keys with `Uuid::new_v4()` | Share a key, row, bucket path or username between trials |
| Interpolate the actual values into every assert message | `assert!(result.is_ok())` with no message |
| Use a container when the behaviour lives in the service | Mock the service and assert the mock returns what you set |
| Run the suite against the in-memory implementation too | Ship a double no suite holds to the contract |
| Gate a test-only double behind `test-utils` + `required-features` | Add a self dev-dependency `mycrate = { path = "." }` — it breaks builds with `colliding StableCrateId` |
| Keep suite-facing constants in `trait_tests*` | Define them in a backend file and have the suite reach in |
| `include_str!` assets from this crate's own `tests/assets/` | Reach into `infrastructure/` or a sibling crate |
| Use `trials_shared` only when every subject is `&impl` and building is expensive | Use it with a `&mut` or owned subject — it is not generated |

Keep a backend file free of comments beyond its `//!` line. Every one of these files has the same
shape, so prose about it is noise. Facts the code cannot carry — a pinned image tag, a load-bearing
provisioning step — belong in `tests/README.md`.

## 4. Checklist

```bash
.claude/skills/backend-trait-test/scripts/check_trait_tests.sh <crate>
cargo test -p <crate> --all-features
cargo test -p <crate> --test <backend> -- --list    # trial count matches expectations
```

`--all-features` is not optional. A binary carrying `required-features` is skipped without it, and
cargo says nothing.

- [ ] No `#[test_trait]` names a concrete type.
- [ ] Running the suite twice in a row still passes.
- [ ] Each test would fail if the behaviour it names broke. Delete any that only restates the
      double's setup.
- [ ] Every type in `src/backends/` has a file in `tests/backends/` and a `[[test]]` stanza.
- [ ] The in-memory implementation / `testing::Mock*` double has one too, and is reachable by downstream crates (plain backend, or `test-utils` feature + `pub mod testing`).
- [ ] Each `tests/backends/*.rs` follows the §2 order, is comment-free apart from its `//!` line, and declares nothing `pub`.
- [ ] Tests are meaningful: each would fail if the behaviour it names broke. Delete any that only restates the double's setup.
- [ ] `Cargo.toml` has `autotests = false` and one stanza per backend file.
- [ ] `tests/README.md` matches the tree on disk.
