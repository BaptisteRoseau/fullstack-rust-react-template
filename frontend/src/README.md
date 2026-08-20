# src

Layer-first source tree. Dependencies flow downwards only; see
[`../docs/architecture/README.md`](../docs/architecture/README.md) for the full rules.

| Folder | Role | May import |
|---|---|---|
| `api/` | Endpoint declarations and SWR services | nothing from the UI layers |
| `design-system/` | Domain-agnostic UI primitives | `utils/`, `types/`, `css/`, `hooks/`, Radix |
| `components/` | Domain-aware shared components | `design-system/`, `api/service/`, `contexts/`, `hooks/`, `stores/` |
| `layouts/` | Page shells rendered by the router | `components/`, `design-system/` |
| `pages/` | One folder per route, private to itself | everything |
| `router/` | Route objects and `PATHS` | `pages/`, `layouts/`, `components/` |
| `hooks/` | Reusable hooks with no domain knowledge | `utils/`, `types/` |
| `utils/` | Pure helpers, no JSX and no hooks | `types/`, `constants/` |
| `types/` | Cross-layer TypeScript types | — |
| `constants/` | App-wide constants that are not routes | — |
| `contexts/` | Scoped React contexts | `utils/`, `types/` |
| `stores/` | Zustand stores for app-wide UI state | `constants/`, `i18n/` |
| `css/` | Global SCSS: tokens, themes, mixins, reset | — |
| `config/` | Validated environment variables | — |
| `i18n/` | Lingui instance and PO catalogs | — |
| `img/` | Static images and SVGs | — |
| `test-utils/` | Render helpers, MSW mocks, fixtures | everything |

`App.tsx` mounts the router, `Context.tsx` is the single provider tree (error boundary, i18n, SWR,
notifications), and `main.tsx` is the runtime bootstrap.
