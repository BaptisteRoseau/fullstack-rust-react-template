---
name: backend-config-entry
description: Use when adding, renaming or removing a configuration setting of the Rust backend (crates/config).
---

# Add a backend configuration entry

A setting must be declared in three files before it reaches the backend. Miss one and the setting
parses fine but is never used, with no compiler error.

Read [crates/config/README.md](../../../crates/config/README.md) first for the `CliConfig` /
`Config` split.

## 1. Declare the default

In [defaults.rs](../../../crates/config/src/defaults.rs), add one constant named `DEFAULT_<SETTING>`.
Every setting has a default.

```rust
pub(crate) const DEFAULT_PROMETHEUS_IP: IpAddr = LOCALHOST;
```

Never put a real secret in a default.

## 2. Add the CLI argument

In [cli.rs](../../../crates/config/src/cli.rs), add a `pub(crate)` field to `CliConfig`.

```rust
/// The port where to bind the server
#[arg(short, long, env, default_value_t = DEFAULT_PORT)]
pub(crate) port: u16,
```

- Always pass `env` and `default_value_t`.
- Add `short` and `long` only when a human is expected to type the flag.
- Prefix the field with its sub-config name: `database_`, `s3_`, `authenticator_`.
- Write the doc comment for a user: it becomes the `--help` text.

## 3. Read it into `Config`

In [config.rs](../../../crates/config/src/config.rs):

1. Add the field to the matching sub-struct, or create a new sub-struct for a new area.
2. Map it in `TryFrom<CliConfig> for Config`.

```rust
api: ApiConfig {
    timeout_sec: value.api_timeout_sec,
},
```

If the feature it configures can be switched off, make the sub-config an `Option` and build it only
when the feature is on. Copy the `prometheus` block in the same file.

## 4. Validate, only when there is a rule

Edit `Config::validate` only if the setting can conflict with another one:

- Incompatible or malformed values: return a `ConfigParsingError`.
- Value provided but ignored: `warn!` and continue.

Skip this step otherwise.

## Checklist

```bash
cargo run -p backend -- --help | grep <your-flag>        # the flag is exposed
grep -n '<your_field>' crates/config/src/config.rs       # the value reaches Config
```

The second grep is the one that matters. An unused `DEFAULT_*` constant is caught by the compiler's
`dead_code` lint, but a `CliConfig` field that is parsed and never mapped into `Config` compiles
without a single warning.

- [ ] The doc comment reads as user-facing help, not as an implementation note.
- [ ] The default is safe to ship (no real secret, no production host).
