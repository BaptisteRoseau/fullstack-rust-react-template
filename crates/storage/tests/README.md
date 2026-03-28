# Storage integration tests

Contains

- a MinIO testcontainer to be reused for the tests (`common/containers.rs`)
- a test suite for the Storage trait to be reused for backends (`common/storage.rs`)
- assets for testing with images for example (`assets/`)

## Running

```sh
cargo test -p storage
```
