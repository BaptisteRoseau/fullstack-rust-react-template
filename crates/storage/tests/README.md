# Storage integration tests

```txt
tests
├── assets
│   └── garage.toml   # Garage server configuration copied into the container
├── backends
│   └── s3.rs         # the Garage testcontainer fixture, and the `s3` test binary
└── trait_tests.rs    # the `#[test_trait_suite]` module for the Storage trait
```

The suite is written once against the `Storage` trait. `backends/s3.rs` says how its
environment starts, pulls the suite in with `#[path = "../trait_tests.rs"]`, and gets
its `fn main()` from `test_trait_main!`. A second backend is a second file there plus
a `[[test]]` stanza in `Cargo.toml`.

## The Garage fixture

Three things in `start_container()` are load-bearing and easy to mistake for ceremony:

- Unlike MinIO, Garage serves no data until a cluster layout is assigned and applied.
- Garage keys are not user/password pairs. The fixture creates one and reads the
  generated key id and secret back out of the CLI output.
- `assets/garage.toml` is a test fixture, not a deployment manifest — Garage is not part
  of the infrastructure, it only backs these tests as an S3-compatible server. Its
  `s3_region` matters: Garage enforces it in request signatures.

## Running

```sh
cargo test -p storage
```

## Skills

- [backend-trait-test](../../../.claude/skills/backend-trait-test/SKILL.md)
- [backend-feature-gating](../../../.claude/skills/backend-feature-gating/SKILL.md)
