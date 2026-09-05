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
- A layer-first React frontend (SWR, SCSS Modules, Radix UI) with routing, i18n, forms,
  MSW mocks, Storybook and tests — see [frontend/docs/architecture](./frontend/docs/architecture).
- Metrics, dashboards, structured logging, an OpenAPI document and Swagger UI.
- An MCP endpoint exposing part of the backend to assistants as callable tools —
  see [crates/mcp](./crates/mcp).
- Scripts for building, linting, unit tests, coverage, CVE and license checks, plus git hooks.

Remaining work is tracked in [TODO.md](./TODO.md).

## Requirements

| Requirement                                   | Version             | Used for                                           |
| --------------------------------------------- | ------------------- | -------------------------------------------------- |
| Rust toolchain (`cargo`, `rustfmt`, `clippy`) | 1.97+, edition 2024 | Backend and tools                                  |
| Docker + Compose                              | 24+                 | Infrastructure, integration tests (testcontainers) |
| Bun                                           | 1.22+               | Frontend package manager, dev server and tests     |
| Node.js                                       | 20+                 | Tooling that Bun shells out to                     |

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

The frontend's `.env.example` points `VITE_APP_API_URL` at the backend **origin**,
`http://localhost:8080`, which is where `cargo run -p backend` listens by default (`--port` /
`PORT` to change it). The `/api` prefix is added by the frontend's API client, not by the variable.

Before pushing, run the same checks the hooks run:

```bash
./scripts/test_lint.sh    # clippy + eslint
./scripts/test_units.sh   # cargo test --workspace --all-features + bun test
./scripts/test_openapi.sh # the committed frontend SDK still matches the api crate
./scripts/git_hooks/setup.sh   # install the pre-push hook once
```

Useful local endpoints once the stack is up. Container ports come from `.env.dev` and can all be
remapped there — note that `GRAFANA_PORT` defaults to `3000` and therefore clashes with the Vite
dev server if you run both:

| Service                          | URL                     | Variable          |
| -------------------------------- | ----------------------- | ----------------- |
| Frontend (Vite dev server)       | <http://127.0.0.1:3000> | —                 |
| Backend (`cargo run -p backend`) | <http://127.0.0.1:8080> | `--port` / `PORT` |
| Frontend (container)             | <http://127.0.0.1:8080> | `FRONTEND_PORT`   |
| Backend (container)              | <http://127.0.0.1:9876> | `BACKEND_PORT`    |
| Swagger UI                       | <http://127.0.0.1:7070> | `SWAGGER_PORT`    |
| MCP endpoint (on the backend)    | <http://127.0.0.1:8080/mcp> | `--mcp-path`  |
| Homepage (index of all services) | <http://127.0.0.1:3002> | `HOMEPAGE_PORT`   |
| Grafana                          | <http://127.0.0.1:3000> | `GRAFANA_PORT`    |
| Prometheus                       | <http://127.0.0.1:9090> | `PROMETHEUS_PORT` |
| pgAdmin                          | <http://127.0.0.1:3001> | `PGADMIN_PORT`    |
| Keycloak                         | <http://127.0.0.1:8090> | —                 |
| MailHog (web UI)                 | <http://127.0.0.1:8030> | —                 |
| SeaweedFS (S3 gateway)           | <http://127.0.0.1:9000> | `S3_PORT`         |
| PostgreSQL                       | `127.0.0.1:5432`        | `POSTGRES_PORT`   |
| Redis                            | `127.0.0.1:6379`        | `REDIS_PORT`      |

## Overview

Each directory has its own `README.md` describing what belongs there and the rules that hold
inside it. Read it before working there, together with those of its parents.

```txt
.
├── crates/            # The Rust backend, one crate per layer or capability
├── frontend/          # React 19 + TypeScript single-page app
├── infrastructure/    # Dockerfiles, Compose fragments, Keycloak realm, Prometheus config
├── scripts/           # build_*.sh and test_*.sh helpers, and the git hooks
├── tools/             # Standalone crates that are more than a script
├── doc/               # Cross-cutting architecture documentation
└── docker-compose.yml # Includes every infrastructure fragment
```

| Where | Start with |
| --- | --- |
| Backend | [crates/README.md](./crates/README.md) |
| Frontend | [frontend/README.md](./frontend/README.md) |
| Architecture deep dives | [doc/README.md](./doc/README.md) |
| Frontend architecture | [frontend/docs/architecture](./frontend/docs/architecture/README.md) |
| Build, test and lint | [scripts/README.md](./scripts/README.md) |

### Software Stack

#### Backend

| Tool                | Role                                                           |
| ------------------- | -------------------------------------------------------------- |
| Rust (edition 2024) | Backend language for everything server-side and the tools      |
| Axum + Tower        | HTTP server, routing, middlewares, rate limiting               |
| Tokio               | Async runtime                                                  |
| SQLx                | Compile-time checked SQL and database migrations               |
| Clap                | CLI and environment-variable configuration                     |
| utoipa + Swagger UI | OpenAPI document generation and interactive docs               |
| rmcp                | Model Context Protocol server, over Streamable HTTP            |
| thiserror / anyhow  | Typed error enum per crate; anyhow in `api` and the binary     |
| tracing             | Structured, span-aware logging (compact or JSON)               |
| testcontainers      | Integration tests against real Postgres and Keycloak instances |

#### Frontend

| Tool | Role |
| --- | --- |
| TypeScript | Frontend language, strict typing throughout |
| React 19 | UI library |
| React Router | Routing and lazy route registration |
| Vite | Dev server and production bundler |
| Bun | Package manager, test and script runner |
| SWR | Server-state fetching, caching and revalidation |
| `@hey-api/openapi-ts` | Generates the typed API client from the backend's OpenAPI document |
| SCSS Modules | Component-scoped styling over shared design tokens |
| clsx | Conditional class composition |
| Radix UI | Unstyled, accessible behaviour for interactive primitives |
| Zustand | App-wide UI state such as notifications and theme |
| React Hook Form + Zod | Forms and schema validation |
| Lingui | i18n extraction and the en/fr catalogs |
| Vitest + Testing Library | Unit and integration tests |
| MSW | API mocking for dev, tests and e2e |
| Playwright | End-to-end tests |
| Storybook | Component development and documentation |
| ESLint + Prettier | Linting and formatting |

#### Infrastructure

| Tool                           | Role                                                   |
| ------------------------------ | ------------------------------------------------------ |
| Docker + Compose               | Local environment and production images                |
| PostgreSQL                     | Primary datastore, with read-write and read-only roles |
| Redis                          | Cache and short-lived shared state                     |
| SeaweedFS                      | S3-compatible object storage for blobs and files       |
| Keycloak                       | Identity provider (OIDC, Authorization Code + PKCE)    |
| Prometheus + postgres_exporter | Metrics collection for the backend and the database    |
| Grafana                        | Dashboards over the Prometheus metrics                 |
| pgAdmin                        | Database inspection during development                 |
| MailHog                        | Captures outgoing mail in development                  |
| Homepage                       | Index page linking to every local service              |

## License

This project is source-available under a [custom commercial license](./LICENSE).

- **Free** for any company whose all-time revenue has not yet reached $20,000 USD (adjusted for inflation from 2026).
- **Early license** available for a one-time fee of $5,000 USD before reaching that threshold.
- **Commercial license** required once the threshold is crossed: one-time fee of max($5,000, 25% of all-time revenue) at the time of payment.

Contact <broseau@gmail.com> to purchase a license.
