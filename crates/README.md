# Crates

This folder contains the different crates that make the backend.

Most importantly:

- [api](./api): The API layer, all HTTP endpoints, middlewares and RBAC.
- [core](./core): The domain layer, where the business logic happens.
- [database](./database): The database layer, exposes traits and API to insterract with the database.

The module relashionship should be `api` > `core` > `database`. Each outer layer cannot import inner layer to keep

## Types of crates

Each crate is this folder is library only. For binaries, go into [binaries](./binaries) and import the required libraries.
