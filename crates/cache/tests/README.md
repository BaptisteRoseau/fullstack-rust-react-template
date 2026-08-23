# Cache integration tests

```txt
tests
├── backends
│   ├── hash_map.rs   # `HashMapCache`, no container, instant
│   └── redis.rs      # the Redis testcontainer fixture, and the `redis` binary
└── trait_tests.rs    # the `#[test_trait_suite]` module for the Cache trait
```

The smallest complete example of the pattern the `backend-trait-test` skill documents:
the suite is written once against the `Cache` trait, and each file in `backends/` is a
whole `harness = false` binary that pulls it in with `#[path = "../trait_tests.rs"]`.

`HashMapCache` is a real backend, not a double, and it is what downstream crates' unit
tests run on — so it has to satisfy the same contract as `Redis`, including `timeout_s`
expiry.

## Running

```sh
cargo test -p cache
```

## Skills

- [backend-trait-test](../../../.claude/skills/backend-trait-test/SKILL.md)
- [backend-feature-gating](../../../.claude/skills/backend-feature-gating/SKILL.md)
