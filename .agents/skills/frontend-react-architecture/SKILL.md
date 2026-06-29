---
name: frontend-react-architecture
description: The global architecture, layering rules, and conventions of the React frontend (bulletproof-react). Use this when starting any frontend work, deciding where code belongs, or reviewing whether a change respects the module boundaries.
---

# Frontend Architecture

A single React 19 + TypeScript SPA under `frontend/`, bundled by Vite, following the
**bulletproof-react** feature-based layout with a strictly **unidirectional** dependency flow.

## The dependency rule (non-negotiable)

Code flows one way only: **shared → features → app**. Enforced by ESLint `import/no-restricted-paths`.

- `app/` may import from `features/` and shared modules.
- `features/*` may import from shared modules but **NEVER from another feature**. Compose features at the `app` layer.
- Shared modules (`components/`, `hooks/`, `lib/`, `types/`, `utils/`, `config/`) import only from each other / down.

If two features need to share code, lift it into a shared module — do not cross-import.

## Where things go

| You are adding… | Put it in… | Skill |
|---|---|---|
| A route/screen | `src/app/pages/**` + `config/paths.ts` + `app/router.tsx` | `frontend-react-page` |
| A vertical slice (domain) | `src/features/<name>/` | `frontend-react-feature` |
| A data fetch/mutation | `src/features/<name>/api/<verb>-<noun>.ts` | `frontend-react-api` |
| A reusable UI primitive | `src/components/ui/<name>/` | `frontend-react-component` |
| A page shell | `src/components/layouts/` | `frontend-react-layout` |
| A reusable hook | `src/hooks/use-*.ts` or feature `hooks/` | `frontend-react-hook` |
| A form | compose `src/components/ui/form` | `frontend-react-form` |
| Global client state | Zustand `create(...)` store | `frontend-react-state` |
| Access control | `src/lib/authorization.tsx` | `frontend-react-authorization` |
| Translatable strings | Lingui macros | `frontend-react-i18n` |
| A test | colocated `__tests__/` | `frontend-react-testing` |
| A mock endpoint | `src/testing/mocks/handlers/` | `frontend-react-mocks` |

## Hard conventions

- **Imports:** absolute via the `@/` alias (`@/components/ui/button`). Relative only inside the same feature/component folder.
- **Files & folders:** `kebab-case`. Component exports `PascalCase`. Functions/vars `camelCase`. Hooks `use-*.ts`.
- **No feature barrels** — import the concrete file (`../api/create-discussion`). Only `components/ui/*` keep an `index.ts` barrel.
- **The single API client** lives in `src/lib/api-client.ts` (the configured Axios `api`). Never call `axios` or `fetch` directly in a component — always go through a feature `api/` hook.
- **Server state = React Query. Client state = Zustand. Local state = useState.** Don't globalize prematurely.
- **Routes** are declared once in `config/paths.ts` (`path` + `getHref`); never hardcode a URL.
- **i18n:** wrap user-facing strings in Lingui macros (`<Trans>` / `` t`...` ``).

## Provider stack

`src/app/provider.tsx` is the only composition root for global providers:
`Suspense → ErrorBoundary(MainErrorFallback) → HelmetProvider → QueryClientProvider → Notifications + AuthLoader`.
Add new app-wide providers here, nowhere else.

## Verify before done

```bash
bun run check-types   # tsc --noEmit
bun run lint          # ESLint (boundaries, kebab-case, a11y, tailwind)
bun run test          # Vitest
```
