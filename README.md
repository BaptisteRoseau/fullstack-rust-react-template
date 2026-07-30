# Fullstack Rust React Template

**WARNING**: This project is under a heavy refactor right now and parts of it are broken.

A production-shaped template for a fullstack web application: an **Axum** backend in Rust, a
**React 19 + TypeScript** single-page frontend, and the surrounding infrastructure (Postgres,
Redis, SeaweedFS, Keycloak, Prometheus, Grafana) already wired together in Docker Compose.

It is built out of low-level open source software, with a bias towards a small CPU and memory
footprint: the goal is to clone the repository, implement only your own frontend and backend
logic on top of the existing layers, and deploy an application able to serve hundreds of
concurrent users from a cheap VPS rather than a managed cloud platform.

What you get out of the box:

- A layered backend (`api` → `app_core` → `database`) where each layer is a separate crate and
  every external dependency sits behind a trait with swappable backends.
- Authentication against Keycloak using a Backend-for-Frontend flow (Authorization Code + PKCE,
  httpOnly cookies, the SPA never handles tokens) — see [doc/authentication](./doc/authentication).
- A frontend following the [bulletproof-react](https://github.com/alan2207/bulletproof-react)
  architecture, with routing, i18n, forms, RBAC, MSW mocks, Storybook and tests.
- Metrics, dashboards, structured logging, an OpenAPI document and Swagger UI.
- Scripts for building, linting, unit tests, coverage, CVE and license checks, plus git hooks.

Remaining work is tracked in [TODO.md](./TODO.md).

## Requirements

| Requirement | Version | Used for |
| --- | --- | --- |
| Rust toolchain (`cargo`, `rustfmt`, `clippy`) | 1.97+, edition 2024 | Backend and tools |
| Docker + Compose | 24+ | Infrastructure, integration tests (testcontainers) |
| Bun | 1.22+ | Frontend package manager, dev server and tests |
| Node.js | 20+ | Tooling that Bun shells out to |

Optional, used by individual scripts: `cargo-llvm-cov` (coverage), `cargo-deny` (licenses and
CVEs), `sqlx-cli` (migrations), `markdownlint`, `cspell`.

## Quick Start

```bash
# 1. Environment variables (.env is a symlink to .env.dev by default)
source .env

# 2. Bring up Postgres, Redis, SeaweedFS, Keycloak, Prometheus, Grafana...
docker compose up -d

# 3. Backend — http://127.0.0.1:8080
cargo run -p backend

# 4. Frontend — http://127.0.0.1:3000
cd frontend && cp .env.example .env && bun install && bun run dev
```

The frontend's `.env.example` points `VITE_APP_API_URL` at `http://localhost:8080/api`, which is
where `cargo run -p backend` listens by default (`--port` / `PORT` to change it).

Before pushing, run the same checks the hooks run:

```bash
./scripts/test_lint.sh    # clippy + eslint
./scripts/test_units.sh   # cargo test --workspace --all-features + bun test
./scripts/git_hooks/setup.sh   # install the pre-push hook once
```

Useful local endpoints once the stack is up. Container ports come from `.env.dev` and can all be
remapped there — note that `GRAFANA_PORT` defaults to `3000` and therefore clashes with the Vite
dev server if you run both:

| Service | URL | Variable |
| --- | --- | --- |
| Frontend (Vite dev server) | <http://127.0.0.1:3000> | — |
| Backend (`cargo run -p backend`) | <http://127.0.0.1:8080> | `--port` / `PORT` |
| Frontend (container) | <http://127.0.0.1:8080> | `FRONTEND_PORT` |
| Backend (container) | <http://127.0.0.1:9876> | `BACKEND_PORT` |
| Swagger UI | <http://127.0.0.1:7070> | `SWAGGER_PORT` |
| Homepage (index of all services) | <http://127.0.0.1:3002> | `HOMEPAGE_PORT` |
| Grafana | <http://127.0.0.1:3000> | `GRAFANA_PORT` |
| Prometheus | <http://127.0.0.1:9090> | `PROMETHEUS_PORT` |
| pgAdmin | <http://127.0.0.1:3001> | `PGADMIN_PORT` |
| Keycloak | <http://127.0.0.1:8090> | — |
| MailHog (web UI) | <http://127.0.0.1:8030> | — |
| SeaweedFS (S3 gateway) | <http://127.0.0.1:9000> | `S3_PORT` |
| PostgreSQL | `127.0.0.1:5432` | `POSTGRES_PORT` |
| Redis | `127.0.0.1:6379` | `REDIS_PORT` |

## Overview

Every directory carries its own `README.md` describing its conventions — read it before
working there, along with those of its parents.

```text
.
├── crates/           # The Rust backend, one crate per layer or capability
│   ├── api/          # HTTP layer: Axum routes, extractors, middlewares, OpenAPI
│   ├── app_core/     # Domain layer: business logic, independent of HTTP and SQL
│   ├── database/     # Postgres access via SQLx, migrations and CRUD traits
│   ├── models/       # Shared domain structs used across the layers
│   ├── authenticator/# Identity provider interface + Keycloak backend (BFF, API keys)
│   ├── cache/        # Cache trait with Redis and in-process HashMap backends
│   ├── storage/      # Blob storage trait over S3-compatible backends, with compression
│   ├── compressor/   # Image and blob compression used by the storage layer
│   ├── rbac/         # Roles, scopes and permission checks
│   ├── config/       # Read-only configuration from CLI, env and file (Clap)
│   ├── logging/      # tracing subscriber setup, human-readable or JSON
│   ├── mailer/       # Transactional email sending
│   ├── binaries/     # The only crates with a main.rs — notably `backend`
│   ├── test_trait/   # Harness to run one test suite against every trait backend
│   └── *_derive/     # Proc-macro crates backing database CRUD and the test harness
├── frontend/         # React 19 + TypeScript SPA (bulletproof-react layout)
│   ├── src/          # app/ pages, features/, components/, lib/, hooks/, i18n/
│   ├── e2e/          # Playwright end-to-end tests
│   ├── __mocks__/    # MSW handlers and the in-memory mock database
│   └── generators/   # Plop templates to scaffold components and features
├── infrastructure/   # Everything needed to run the platform in containers
│   ├── docker/       # Dockerfiles for the backend, frontend, Postgres, homepage
│   ├── docker-compose/ # Compose fragments merged by the root docker-compose.yml
│   ├── keycloak/     # Realm export imported on startup
│   └── prometheus/   # Scrape configuration
├── scripts/          # build_*.sh, test_*.sh helpers and the git hooks
├── tools/            # Standalone crates that are more than a script
│   ├── http_health_checker/ # Tiny static binary used as a container healthcheck
│   └── openapi_generator/   # Exports openapi.json offline from the api crate
├── doc/              # Cross-cutting documentation (authentication, refactors)
└── docker-compose.yml # Entry point including every infrastructure fragment
```

### Software Stack

#### Backend

| Tool | Role |
| --- | --- |
| Rust (edition 2024) | Backend language for everything server-side and the tools |
| Axum + Tower | HTTP server, routing, middlewares, rate limiting |
| Tokio | Async runtime |
| SQLx | Compile-time checked SQL and database migrations |
| Clap | CLI and environment-variable configuration |
| utoipa + Swagger UI | OpenAPI document generation and interactive docs |
| thiserror / anyhow | Typed errors per crate, application-level error context |
| tracing | Structured, span-aware logging (compact or JSON) |
| testcontainers | Integration tests against real Postgres and Keycloak instances |

#### Frontend

| Tool | Role |
| --- | --- |
| TypeScript | Frontend language, strict typing throughout |
| React 19 | UI library |
| React Router | Routing, lazy route registration and data loaders |
| Vite | Dev server and production bundler |
| Bun | Package manager and test/script runner |
| TanStack Query | Server-state fetching, caching and mutations |
| Zustand | Global client-side UI state (modals, notifications, theme) |
| React Hook Form + Zod | Forms and schema validation, shared with API response parsing |
| Tailwind CSS + Radix UI | Styling and accessible headless primitives (ShadCN pattern) |
| Lingui | i18n message extraction and en/fr catalogs |
| Vitest + Testing Library | Unit and integration tests |
| MSW | API mocking for dev, tests and e2e |
| Playwright | End-to-end tests |
| Storybook | Component development and documentation |
| ESLint + Prettier | Linting and formatting |

#### Infrastructure

| Tool | Role |
| --- | --- |
| Docker + Compose | Local environment and production images |
| PostgreSQL | Primary datastore, with read-write and read-only roles |
| Redis | Cache and short-lived shared state |
| SeaweedFS | S3-compatible object storage for blobs and files |
| Keycloak | Identity provider (OIDC, Authorization Code + PKCE) |
| Prometheus + postgres_exporter | Metrics collection for the backend and the database |
| Grafana | Dashboards over the Prometheus metrics |
| pgAdmin | Database inspection during development |
| MailHog | Captures outgoing mail in development |
| Homepage | Index page linking to every local service |

## License

See [LICENSE](./LICENSE).
