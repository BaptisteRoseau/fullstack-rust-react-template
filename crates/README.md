# Crates

This folder contains the different crates that make the backend.

Most importantly:

- [api](./api): The API layer, all HTTP endpoints, middlewares and RBAC.
- [core](./core): The domain layer, where the business logic happens.
- [database](./database): The database layer, exposes traits and API to insterract with the database.
- [config](./config): The configuration either from a file or the CLI, with defaults. This config is passed to all the previous layers and is read-only once parsed.
- [storage](./storage): The API to store blobs and files. Consider your environment read-only, every write operation should either be in the database for data, or the storage API for files.

The module relashionship should be `api` > `core` > `database`. Each outer layer cannot import inner layer to keep a coherent architecture and allow working on modules independently.

## Types of crates

Each crate is this folder is library only. For binaries, go into [binaries](./binaries) and import the required libraries.

## Tech Stack

The tech stack used in the backend is:

- Axum and Tower for the API layer
- SQLx for the database layer and migrations
- Clap for CLI interface
- utoipa for the openapi.json and swagger UI