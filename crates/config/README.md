# Config

Every configurable value of the backend. `Config` is built once at startup, then passed read-only to
every other layer.

## Files

| File | Contents |
| --- | --- |
| [defaults.rs](src/defaults.rs) | One `DEFAULT_*` constant per setting. Every setting has one |
| [cli.rs](src/cli.rs) | `CliConfig`: the flat Clap parser for flags, environment variables and the config file |
| [config.rs](src/config.rs) | `Config` and its sub-structs, built from `CliConfig` |
| [error.rs](src/error.rs) | `ConfigParsingError` |

Only `Config` and its sub-structs are public. `CliConfig` stays private to the crate.

## Why two structs

`CliConfig` is **flat**. Every setting stays reachable from a CLI flag, an environment variable or
the config file, with no nesting to spell out.

`Config` is **grouped** into sub-structs such as `ApiConfig` or `PostgresConfig`. An optional
sub-struct is built only when all of its values are present, so client code reads
`config.postgres.host` without re-checking anything.

## Conventions

**Defaults** are constants named `DEFAULT_<SETTING>`, declared in
[defaults.rs](src/defaults.rs) and nowhere else:

```rust
pub(crate) const DEFAULT_PORT: u16 = 8080;
```

**CLI arguments** live in `CliConfig` in [cli.rs](src/cli.rs). Arguments belonging to the same
sub-config share a prefix, and every one of them takes its default from a `DEFAULT_*` constant:

```rust
/// The port where to bind the server
#[arg(short, long, env, default_value_t = DEFAULT_PORT)]
pub(crate) port: u16,
```

**Conversions** all happen in the `TryFrom<CliConfig> for Config` impl in
[config.rs](src/config.rs). It is the only place a `CliConfig` field is read. A sub-config that can
be switched off is an `Option`, built there only when the feature is on.

`Config::validate` runs before the conversion. It returns a `ConfigParsingError` for incompatible
options, and logs a `warn!` for options that are ignored or deprecated.

## Skills

- [backend-config-entry](../../.claude/skills/backend-config-entry/SKILL.md)
