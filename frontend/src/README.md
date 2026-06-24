# `src/`

All application code. The layout is **feature-based** (bulletproof-react) with a **unidirectional**
dependency rule enforced by ESLint: code flows **shared → features → app**.

| Folder        | What lives here                                                                                                   |
| ------------- | ----------------------------------------------------------------------------------------------------------------- |
| `app/`        | Composition root: providers, router, and page components. Only place features are stitched together.              |
| `components/` | Shared, app-agnostic UI: `ui/` design system, `layouts/`, `errors/`, `seo/`.                                      |
| `features/`   | Self-contained domain slices (`auth`, `discussions`, `comments`, `teams`, `users`). **No cross-feature imports.** |
| `lib/`        | Preconfigured singletons: `api-client`, `react-query`, `auth`, `authorization`.                                   |
| `config/`     | `env.ts` (validated env vars) and `paths.ts` (the route source of truth).                                         |
| `hooks/`      | Shared reusable hooks (`use-*.ts`).                                                                               |
| `testing/`    | `renderApp` test utils, MSW mocks, data generators.                                                               |
| `i18n/`       | Lingui setup + `en`/`fr` PO catalogs.                                                                             |
| `types/`      | Shared types — `api.ts` holds the domain models.                                                                  |
| `utils/`      | Pure helpers (`cn`, `format`).                                                                                    |
| `assets/`     | Static assets.                                                                                                    |
| `main.tsx`    | Runtime bootstrap: starts MSW (if enabled) then mounts `<App/>`.                                                  |

## Dependency rule

- `app` may import from `features` and shared modules.
- `features/*` may import from shared modules but **never from each other** — compose them in `app`.
- Shared modules never import from `features` or `app`.

## Conventions

- Imports use the `@/` alias (`@/components/ui/button`). Files/folders are `kebab-case`.
- Server state → React Query (in feature `api/`). Client state → Zustand. Local → `useState`.
- Routes declared in `config/paths.ts`; user-facing strings wrapped in Lingui macros.

See the `.claude/skills/frontend-react-architecture` skill for the full picture, and the per-folder
READMEs for specifics.
