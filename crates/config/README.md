# Config

This is where all configurable variables and constants should be.

Is contains:

- [defaults.rs](src/defaults.rs): Default constants used to initialize the config. Every default has to be set here.
- [cli.rs](src/cli.rs): The command-line interface getters to configure from CLI.
- [config.rs](src/config.rs): The actual `Config` object and its subconfigs used in the rest of the project.

Only `Config` and its inner config structures can be public in this crate.

## Adding a new entry

1. Add the default entry constant, prefixed by `DEFAULT` in [defaults.rs](src/defaults.rs)
2. Add the CLI argument in [cli.rs](src/cli.rs). Add `short` and/or `long` only if it makes sense.
3. Add the conversion from `CliConfig` to `Config` in [config.rs](src/config.rs) in the `try_from` function. Add a new subconfig if that makes sense.
4. (opt) Add validation in the `validate` only if there are restrictions applicables (ex. invalid format, incompatible arguments, argument provided but ignored)

Here are examples of:

- A default:

```rs
pub(crate) const DEFAULT_PROMETHEUS_IP: IpAddr = LOCALHOST;
```

- A CLI argument:

```rs
/// The port where to bind the server
#[arg(short, long, env, default_value_t = DEFAULT_PORT)]
pub(crate) port: u16,
```

- A config conversion:

```rs
Ok(Self{
    debug: value.debug,
    api: ApiConfig {
        timeout_sec: value.api_timeout_sec,
    },
    ...
})
```
