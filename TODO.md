# TODO

## Milestones

### Current

1. [x] Fix SQLx migrations
2. [x] Generate Rust models from SQLx
3. [ ] Build or use a CRUD macro trait for Rust models
    - Make sure keys and values are explicitly stated in the request, not \*, to avoid breaking the app upon table alteration
4. [x] Make a script to generate the models
5. [ ] Add CRUD macros to the models
6. [x] Create a Database trait
7. [ ] Implement CRUD handler in Database trait
8. [ ] Make a script to generate SQLx static file for offline sqlx compilation
9. [ ] Change license to non-commercial

### Database Layer

- [x] Set-up a database with two users: read_write and read_only
- [x] Create the first sqlx migration with a table containing the users
    - [x] Move the created_at/updated_at function to the first migration
- [x] Generate backend Rust database models for the User
- [ ] Make your own dyn-compatible CRUD macro (fork <https://docs.rs/sqlx-crud/latest/sqlx_crud/traits/trait.Schema.html>)
- [x] Use a script to generate the database models: <https://github.com/jayy-lmao/sql-gen?tab=readme-ov-file>
- The two previous milestones should allow you to make the database crate basic CRUD functionalities across all tables trivial, so that you can focus on the more interesting ones :D
- [ ] Use a mix of SeaORM and SQLX ?

### API Layer

- [ ] Use a Protobuf schema to generate Rust models and Typescript structures
- [ ] Add CORS middleware
- [ ] Add compression middleware
- [ ] Add tracing middleware (set sensitive headers before)
- [ ] Add timeout middleware
- [ ] Add Swagger UI & openapi.json
- [ ] Add rate limiter middleware
- [ ] Aggregate middlewares cleanly
- [ ] Convert `core` models to API models and vice-versa
- [ ] Add error handling middleware & global error conversion
- [ ] Trace errors & normalize error response
- [ ] Support both REST & gRCP from the same handler (split with `/rest/` and `/grpc/` in the URL)

### Testing, CI/CD, Docker and scripts

- [ ] Fix all docker images creation
- [ ] Fix all docker-compose files, services & interaction
- [ ] Fix scripts for test execution, audit & licenses
- [ ] Add formatting checker script
- [ ] Add sqlx JSON schema generation from migration scripts and blank container
- [ ] Add sqlx JSON schema checker (current vs expected from migrations)
- [ ] Add protobuf models generation (front & back)
- [ ] Add generated models checker (expected vs actual)
- [ ] Add database crate models generation from sqlx JSON schema
- [ ] Add database crate models generation from sqlx JSON schema checker (expected vs actual)
- [ ] Integrate everything into GitLab CI
- [ ] Integrate everything into GitHub CI
- [ ] Automatically build containers
- [ ] Add Mailhog for local development
- [ ] Add unit & integration tests using testcontainers when necessary
- [ ] Use transaction/rollback in setUp/tearDown for tests

### Security

- [ ] Hasicorp Vault integration to store & rotate secrets

### Frontend

- [ ] SSR vs Client query helper
- [ ] Build React mainstream architecture (component/pages/controllers)

### Core and authentication - API

- [ ] Select authentication service (Supabase ? Keycloak ?)
- [ ] Use tower-auth middleware
- [ ] Use JWT & auto-rotate

### User Management & Information Update (back & front)

- [ ] User Dashboard
- [ ] API to update user information

### Storage layer

- [ ] Select S3-compatible backend service (<https://garagehq.deuxfleurs.fr/> ?)
- [ ] Write the trait: save/load
- [ ] Write middleware that handles file metadata & compression
    - [ ] meta: filename, type, owner & access
    - [x] gzip compression by default
    - [ ] optional encryption
    - [x] caesium image optimizer
    - [ ] pdf file compression
- [ ] Add benchmarks and use testcontainers to set them ups

### Payment Gateway

- [ ] Create crate, write trait
- [ ] Use Stripe integration (frontend embedding + backend IPN)

### Invoices & Payment User information and update

- [ ] Invoice template & builder
- [ ] IPN notification handler
- [ ] Invoice upon instant payment
- [ ] Send invoices by mail automatically
- [ ] Store PDFs into Storage

### Documentation

- [ ] README.md in every directory explaining best practices of said directory
- [ ] `doc/` for developer documentation
- [ ] CLAUDE.md and other LLM templates

### Extras

- [ ] Loki docker plugin to expose docker logs to Grafana
- [ ] Pre-built Grafana dashboards
- [ ] Kubernetes manifests
- [ ] Nix flake (Docker & Prometheus & Kubernetes)
- [ ] Hasicorp Vault for certificates/keys/passwords ?
- [ ] Postgres MCP server using read-only user for IDE & Grafana (if possible)
- [ ] Admin Dashboard ?
