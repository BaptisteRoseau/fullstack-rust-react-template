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
- [ ] Test the audit function

## Milestones

### Database Layer

- [X] Set-up a database with two users: read_write and read_only
- [ ] Create the first sqlx migration with a table containing the users
    - [ ] Move the created_at/updated_at function to the first migration
- [ ] Generate backend Rust database models for the User
- [ ] CRUD macro (see something like <https://docs.rs/sqlx-crud/latest/sqlx_crud/traits/trait.Schema.html>)
- [ ] Use a script to generate the database models: <https://github.com/jayy-lmao/sql-gen?tab=readme-ov-file>
- The two previous milestones should allow you to make the database crate basic CRUD functionalities across all tables trivial, so that you can focus on the more interesting ones :D

### API Layer

### Testing, CI/CD, Docker and scripts

- [ ] Fix all docker images creation
- [ ] Fix all docker-compose files, services & interaction
- [ ] Fix scripts for test execution, audit & licenses
- [ ] Add formating checker script
- [ ] Add sqlx JSON schema generation from migration scripts and blank container
- [ ] Add sqlx JSON schema checker (current vs expected from migrations)
- [ ] Add protobuf models generation (front & back)
- [ ] Add generated models checker (expected vs actual)
- [ ] Add database crate models generation from sqlx JSON schema
- [ ] Add database crate models generation from sqlx JSON schema checker (expected vs actual)
- [ ] Integrate everything into GitLab CI
- [ ] Integrate everything into GitHub CI
- [ ] Automatically build containers

### Core and authentication - API

- [ ] Select authentication service (Supabase ? Keycloak ?)
- [ ] Use tower-auth middleware
- [ ] Use JWT & auto-rotate

### User Management & Information Update (back & front)

### Storage layer

- [ ] Select S3-compatible backend service
- [ ] Write the trait: save/load
- [ ] Write middleware that handles file metadata & compression
    - [ ] meta: filename, type, owner & access
    - [ ] gzip compression by default
    - [ ] optional encryption
    - [ ] caesium image optimizer
    - [ ] pdf file compression

### Payment Gateway

- [ ] Create crate, write trait
- [ ] Use Stripe

### Invoices & Payment User information and update
