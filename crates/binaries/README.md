# Binaries

The only crates allowed a `main.rs`. Everything else in [crates](../README.md) is a library.

Binary crates stay minimal. Logic big enough to name belongs in a library crate, not here.

```txt
binaries/
└── backend/
    ├── src/
    │   ├── main.rs      # parses Config, calls program::run, sets the process exit code
    │   └── program.rs   # the startup sequence: build the backends, serve
    └── Cargo.toml
```

## The `main.rs` contract

Every binary has a `program.rs` beside its `main.rs`, and `main` does only three things: parse the
config, call `program::run`, and turn an error into a non-zero exit code. See
[backend/src/main.rs](./backend/src/main.rs).

`main` owns the `Config` and passes it into `run`. Keeping it at the top level lets it outlive
`run`, so the whole program can read it without lifetime plumbing.

`main` is also the only place that prints to stderr and calls `exit`. Everything below returns a
`Result`.

## Skills

- [backend-config-entry](../../.claude/skills/backend-config-entry/SKILL.md)
