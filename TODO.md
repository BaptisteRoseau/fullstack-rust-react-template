# TODO

- [ ] SQL-based backend for Database
    - [ ] Use as a Drop-in replacement for Postgres/SQlite/MyQSL.. -> Then use SQLite as the database exposed mock
    - [ ] Use a Database mock to replace the @crates/app_core/src/api_key.rs tests

- [ ] Add a mention or a tag in the Rust crates to telle whether they are for testing/traits/utils
- [ ] Make an MCP crate with Rust macros (similar to the API layer)

- [ ] Agents (implementer, reviewer, planner..)

- [ ] Objects for V1:
    - [ ] Users
    - [ ] Api Keys
    - [ ] Invoices
    - [ ] Files

## Error management

- [ ] Include dev debug errors in the response in debug mode (release should not even have the debug field in the response (#[debug_assertion] ?))
- [ ] Add an utoipa model for the rate limiter
- [ ] Add hooks on errors ?

## AI infra & Frontend

- [ ] Cleanup the react-structure.md file
    - [ ] Adapt to target architecture
    - [ ] Split into skills
    - [ ] Mention the skills in the AGENTS.md and README.md
    - [ ] Merge frontend/docs into it

- [ ] Make a skill to use `plop` and adapt [frontend/generators/component] to match the target template
- [ ] Tell existing skills to use `plop`

## Backend

- [ ] Refine error handling to have client-facing and internal errors
    - [ ] Avoid creating new boxes every time we convert an error ?
    - [ ] Find a way to factorize openapi specs for error responses
    - [ ] Create endpoint-specific error response schemas
- [ ] Add the file name to the upload/download endpoint using the file metadata (currently downloading a blob)
- [ ] Maybe use the s3 client directly instead of `Storage` since its interface is so good
- [ ] Standardize API tests

- [ ] Nix & k3s infra

- [ ] Cache uuid <-> username in Redis
- [ ] use username as slug ?
- [ ] Use testcontainers for `can_reach_google` test

- [ ] Snippets/tool to make templates (extractors, db queries etc..)

## Milestones

### Current

- [ ] Avoir un docker compose up qui fonctionne:
    - [x] DB
    - [x] Prometheus
    - [x] Grafana
    - [x] Backend
    - [x] Metrics endpoint
    - [x] Frontend
    - [x] Fix bun run dev
    - [ ] Simple echo API with a simple button (somehow like a TOOD app)
    - [ ] CRUD files s3

- [ ] Generate `frontend/src/types/api.ts` from the openapi of the backend.

1. [x] Fix SQLx migrations
2. [x] Generate Rust models from SQLx
3. [ ] Build or use a CRUD macro trait for Rust models
    - Make sure keys and values are explicitly stated in the request, not \*, to avoid breaking the app upon table alteration
4. [x] Make a script to generate the models
5. [ ] Add CRUD macros to the models
6. [x] Create a Database trait
7. [ ] Implement CRUD handler in Database trait
8. [x] Make a script to generate SQLx static file for offline sqlx compilation
9. [ ] Change license to non-commercial

### Database Layer

- [x] Set-up a database with two users: read_write and read_only
- [x] Create the first sqlx migration with a table containing the users
    - [x] Move the created_at/updated_at function to the first migration
- [x] Generate backend Rust database models for the User
- [ ] Make your own dyn-compatible CRUD macro (fork <https://docs.rs/sqlx-crud/latest/sqlx_crud/traits/trait.Schema.html>)
- [x] Use a script to generate the database models: <https://github.com/jayy-lmao/sql-gen?tab=readme-ov-file>
- [x] Set up a database with two users: read_write and read_only
- [x] Create the first sqlx migration with a table containing the users
    - [x] Move the created_at/updated_at function to the first migration
- [X] Generate backend Rust database models for the User
- [X] CRUD macro (see something like <https://docs.rs/sqlx-crud/latest/sqlx_crud/traits/trait.Schema.html>)
- [X] Use a script to generate the database models: <https://github.com/jayy-lmao/sql-gen?tab=readme-ov-file>
- The two previous milestones should allow you to make the database crate basic CRUD functionalities across all tables trivial, so that you can focus on the more interesting ones :D
- [X] Split database into multiple traits then use T1 + T2 + T3 .. to avoid having a giant one.
- [ ] Atomic transactions support (transactions)

### API Layer

- [x] Set a request ID for logging purposes
- [ ] Use slugs instead of IDs whenever possible
- [ ] Use a Protobuf schema to generate Rust models and Typescript structures
- [ ] Implement an actual CORS middleware
- [x] Add compression middleware
- [x] Add tracing middleware (set sensitive headers before)
- [x] Add timeout middleware
- [x] Add Swagger UI & openapi.json
    - [X] Add categories in the Swagger UI (Auth, Storage...)
    - [X] Add auth support and auth documentation in the swagger UI
    - [ ] Fix the OIDC documentation (currently not usable)
- [X] Add rate limiter middleware
- [X] Aggregate middlewares cleanly
- [ ] Convert `app_core` models to API models and vice-versa
- [x] Add error handling middleware & global error conversion
- [x] Trace errors & normalize error response
- [ ] Support both REST & gRCP from the same handler (split with `/rest/` and `/grpc/` in the URL)
- [ ] Add an MCP crate (similar to the API crate), merge it in the router

### Testing, CI/CD, Docker and scripts

- [ ] Fix all docker images creation
- [X] Fix all docker-compose files, services & interaction
- [ ] Fix scripts for test execution, audit & licenses
- [ ] Add formatting checker script
- [x] Add sqlx JSON schema generation from migration scripts and blank container
- [ ] Add sqlx JSON schema checker (current vs expected from migrations)
- [ ] Add protobuf models generation (front & back)
- [ ] Add generated models checker (expected vs actual)
- [ ] Add database crate models generation from sqlx JSON schema
- [ ] Add database crate models generation from sqlx JSON schema checker (expected vs actual)
- [ ] Integrate everything into GitLab CI
- [ ] Integrate everything into GitHub CI
- [ ] Automatically build containers
- [ ] Fuzz-testing from the OpenAPI spec
- [ ] Build documentation using redocly
- [x] Add Mailhog for local development
- [ ] Add unit & integration tests using testcontainers when necessary
- [ ] Use transaction/rollback in setUp/tearDown for tests

### Security

- [ ] Hasicorp Vault integration to store & rotate secrets
- [ ] Harden Keycloak realms

### Frontend

- [ ] SSR vs Client query helper
- [ ] Build React mainstream architecture (component/pages/controllers)

### Core and authentication - API

- [x] Select authentication service (Keycloak)
- [X] Use JWT & auto-rotate

### User Management & Information Update (back & front)

- [ ] User Dashboard
- [ ] API to update user information (/me, not /user/id)

### Storage layer

- [x] Select S3-compatible backend service (Seaweedfs)
- [ ] Write the trait
    - [ ] full fledge s3 API
    - [ ] Allow the selection of a bucket
    - [ ] Create fake IDs that can be serialized/deserialized easily (base64 ?)
- [ ] Handle ownership and access
- [ ] Write middleware that handles file metadata & compression
    - [ ] meta: filename, type, owner & access
    - [x] gzip compression by default
    - [x] caesium image optimizer
    - [ ] pdf file compression
- [ ] Add benchmarks and use testcontainers to set them ups

### Payment Gateway

- [ ] Create crate, write trait
- [ ] Use Stripe integration (frontend embedding + backend IPN)

### Invoices & Payment User information and update

- [ ] Invoice template & builder
- [ ] European invoice API integration
- [ ] IPN notification handler
- [ ] Invoice upon instant payment
- [ ] Send invoices by mail automatically
- [ ] Store PDFs into Storage

### Documentation

- [ ] README.md in every directory explaining best practices of said directory
- [ ] Skills for everything that is satisfying enough long-term
    - [X] Endpoint writing
    - [X] Config entries
- [ ] `doc/` for developer documentation
- [X] CLAUDE.md and other LLM templates

### Extras

- [ ] Add a management CLI binary -> bound to `api_core` handlers
- [ ] Loki docker/k3s plugin to expose docker logs to Grafana
- [ ] Pre-built Grafana dashboards
- [ ] Kubernetes manifests
- [ ] Nix flake (Docker & Prometheus & Kubernetes)
- [ ] Hasicorp Vault for certificates/keys/passwords ?
- [ ] Agent integration (MCP Gateway ?) read-only for every service (Backend, Postgres)
- [ ] Admin Dashboard ? -> Requires SSR to "display X only if user Y can"
