# Database integration tests

```txt
tests
├── backends
│   ├── postgres.rs   # the Postgres testcontainer fixture, and the `postgres` binary
│   └── mock.rs       # `database::testing::MockDatabase`, held to the same contract
└── trait_tests.rs    # the `#[test_trait_suite]` module for the Database trait
```

The suite's subject is `&mut impl Database`, so each trial gets its own instance and
`trials_shared` is not generated.

## The Postgres fixture

`start()` runs `migrations/` against the container, so the tests see the production
schema. The production image provides `uuidv7()` through an extension the test image
does not have; `UUIDV7_INIT_SQL` installs a compatible pure-SQL version at startup so
the migrations run unmodified.

## The double

`MockDatabase` is what downstream crates' unit tests run on, so it has to satisfy the
same contract as `Postgres`. It is feature-gated (`test-utils`), and an integration test
links this crate the way a consumer does, so its binary carries
`required-features = ["test-utils"]` — a bare `cargo test -p database` silently skips it.

## Running

```sh
cargo test -p database --all-features
```

## Skills

- [backend-trait-test](../../../.claude/skills/backend-trait-test/SKILL.md)
- [backend-feature-gating](../../../.claude/skills/backend-feature-gating/SKILL.md)
