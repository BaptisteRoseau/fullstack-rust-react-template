# Storage integration tests

Contains

- a Garage testcontainer to be reused for the tests (`common/containers.rs`)
- a test suite for the Storage trait to be reused for backends (`common/storage.rs`)
- the Garage server configuration copied into that container (`assets/garage.toml`)

## Running

```sh
cargo test -p storage
```
