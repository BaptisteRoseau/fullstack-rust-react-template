# Frontend

React SPA for the fullstack Rust/React template.

## Getting started

```bash
bun install
cp .env.example .env
bun run dev
```

The app serves on <http://localhost:3000> and talks to the backend at `VITE_APP_API_URL`
(`http://localhost:8080` by default — see `docker compose up -d` and `cargo run -p backend` at the
repository root).

To run without a backend, start the mock API instead:

```bash
bun run run-mock-server                 # Express + MSW handlers on port 8081
VITE_APP_API_URL=http://localhost:8081 bun run dev
```

## Architecture

**Layer-first**: top-level folders under `src/` are technical roles — `api/`, `design-system/`,
`components/`, `pages/`, `hooks/`, `utils/` — and dependencies flow downwards only.

```
pages/ → components/ → design-system/ → hooks/ · utils/ · types/ · css/
                    ↘ api/
```

Stack: Vite, React Router, **SWR** for server state, **SCSS Modules** + design tokens for styling,
**Radix UI** for accessible behaviour, Zustand for app-wide UI state, React Hook Form + Zod for
forms, Lingui for i18n, Vitest + Testing Library + MSW + Playwright for tests.

The canonical reference is [`docs/architecture/`](docs/architecture/README.md). Start with the
[overview](docs/architecture/README.md), then the section you need:

| Doc | Covers |
|---|---|
| [01 – API layer](docs/architecture/01-api.md) | endpoint declarations, SWR services, mocks |
| [02 – Design system](docs/architecture/02-design-system.md) | primitives, SCSS Modules, Radix, stories |
| [03 – Shared components](docs/architecture/03-components.md) | domain-aware components, forms |
| [04 – Pages & router](docs/architecture/04-pages-router.md) | routes, layouts, page folders |
| [05 – Hooks, utils & types](docs/architecture/05-hooks-utils-types.md) | the bottom layers |
| [06 – Tooling](docs/architecture/06-tooling.md) | styles, state, i18n, tests, linting |

Agents: the same material is packaged as skills — `frontend-architecture`, `frontend-api`,
`frontend-design-system`, `frontend-component`, `frontend-page`, `frontend-form`, `frontend-state`,
`frontend-i18n`, `frontend-mocks`, `frontend-testing`, `frontend-seo`.

## Commands

| Command | What it does |
|---|---|
| `bun run dev` | Vite dev server on port 3000 |
| `bun run build` | Type-check then production build |
| `bun run check-types` | `tsc --noEmit` |
| `bun run lint` | ESLint over `src` and `e2e` |
| `bun run format` | Prettier write |
| `bun run test` | Vitest, single run |
| `bun run test:watch` | Vitest, watch mode |
| `bun run test:e2e` | Playwright; starts the dev and mock servers itself |
| `bun run storybook` | Storybook on port 6006 |
| `bun run run-mock-server` | Standalone MSW API server |
| `bun run i18n:extract` | Scan sources into the PO catalogs |
| `bun run i18n:compile` | Compile catalogs for the runtime |
| `bun run i18n:check` | CI gate: extraction clean, every message translated |
| `bun run generate` | Plop scaffolding — see [`generators/`](generators/README.md) |

## Environment

| Variable | Meaning |
|---|---|
| `VITE_APP_API_URL` | Backend origin, no trailing path (`http://localhost:8080`) |
| `VITE_APP_ENABLE_API_MOCKING` | Start the MSW browser worker instead of hitting the backend |
| `VITE_APP_URL` | The app's own origin, used by mock redirects |
| `VITE_APP_MOCK_API_PORT` | Port for `run-mock-server` |

Only variables prefixed `VITE_APP_` reach the client; they are validated by `src/config/env.ts`.

## Authentication

Authentication is a backend-for-frontend OIDC flow against Keycloak. "Log in" and "Register" send
the browser to `${VITE_APP_API_URL}/api/auth/{login,register}?redirect=<path>`; the backend performs
the OAuth exchange and stores the tokens in httpOnly cookies. The frontend never sees a token — it
reads `/api/auth/me`, calls `/api/auth/logout`, and `apiFetch` retries once through
`/api/auth/refresh` when a request comes back 401.
