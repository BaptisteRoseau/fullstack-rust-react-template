# TODO

- [ ] infrastructure
    - [ ] docker
        - [ ] migrations
        - [ ] mailhog
    - [ ] compose
- [ ] docker compose
- [X] crates:
    - [X] api
    - [X] binaries
    - [X] core
    - [X] database:
        - [X] migrations (sqlx)
- [ ] Hasicorp Vault ?
- [ ] Migrate existing crates
- [ ] scripts
- [ ] doc
- [ ] README.md
- [ ] Provision .env.dev
- [ ] Nix flake ?
- [ ] Make a shell script with key/values to ensure dependencies restrictions are followed (database cannot use "core" or "api" as its dependency)
- [ ] Script to generate SQL -> Rust models (use SQL, this might already exist)
- [ ] Script to generate API Rust/Typescript models (there should be a tool to do so from a schema file)
- [ ] Support both REST and gRCP in the same endpoint

## Frontend

- [ ] Make a clear difference between "client_query" and "ssr_query" in the frontend.

## RBAC

Entities that can have permissions:

- Users
- Groups
- Roles

Permissions

Entities can define their permissions ?

## SQL

- [X] Add a trigger on "create table" like this one:
- [ ] Test the trigger
- [X] Add an audit function to make sure
- [ ] Test the trigger

## Milestones

- [ ] Set-up a database with two users: read_write and read_only
- [ ] Create the first sqlx migration with a table containing the users
- [ ] Generate backend Rust database models for the User
- [ ] Generate database CRUD