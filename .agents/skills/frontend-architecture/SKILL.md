---
name: frontend-architecture
description: The layer-first architecture, layering rules, stack and file conventions of the React frontend (SWR + SCSS Modules + Radix). Use this when starting any frontend work, deciding where a file belongs, or reviewing whether a change respects the layer boundaries.
---

# Frontend architecture

`frontend/` is a **layer-first** React SPA: top-level folders are technical roles, not domains.
The canonical reference lives in `frontend/docs/architecture/`; this skill is the working summary.

## Stack

| Concern | Choice |
|---|---|
| Bundler | Vite (single SPA entry) |
| Routing | React Router, `createBrowserRouter` |
| Server state | SWR — global `fetcher` set once in `src/Context.tsx` |
| HTTP | `fetch` wrapper in `src/api/client.ts` — no Axios |
| Global UI state | Zustand (`src/stores/`) |
| Styling | SCSS Modules + design tokens (`src/css/`) — no Tailwind |
| Accessible behaviour | Radix UI primitives, we own the CSS |
| Class composition | `clsx` — no `cva`, no `tailwind-merge` |
| Forms | React Hook Form + Zod |
| i18n | Lingui macros |
| Tests | Vitest + Testing Library, MSW, Playwright |

## Directory tree

```
src/
├── api/            # endpoint declarations + SWR services
├── components/     # domain-aware shared components
├── config/         # env.ts
├── constants/
├── contexts/
├── css/            # tokens, mixins, reset, main.scss
├── design-system/  # domain-agnostic UI primitives
├── hooks/
├── i18n/
├── img/
├── layouts/        # AppLayout, AuthLayout, ContentLayout
├── pages/          # one folder per route
├── router/         # routes.tsx, constants.ts
├── stores/         # Zustand
├── test-utils/     # render helpers, MSW mocks, fixtures
├── types/
├── utils/
├── App.tsx
├── Context.tsx
└── main.tsx
```

## Layering rules

Dependencies flow **downwards only**:

```
pages/ → components/ → design-system/ → hooks/ · utils/ · types/ · css/
                    ↘ api/
```

- `design-system/` may import `utils/`, `types/`, `css/`, `hooks/`, Radix. It may **never** import
  `api/`, `contexts/`, `components/`, `pages/` or `layouts/`. A primitive takes data as props.
- `components/` may import `design-system/`, `api/service/`, `contexts/`, `hooks/`. Domain
  awareness is exactly what distinguishes it from the design system.
- `pages/` may import everything. Routing concerns appear only here and in `router/`.
- `api/` imports nothing from the UI layers.
- **No page imports another page.** A page folder is private; its `index.ts` is its only surface.
  Shared UI moves up into `components/`.

Both boundaries are enforced by `no-restricted-imports` in `eslint.config.cjs`. If you need to
relax one, you are probably putting the file in the wrong layer.

## File naming

| Pattern | Meaning |
|---|---|
| `ComponentName/ComponentName.tsx` | React component; folder name matches file name |
| `ComponentName.test.tsx` | Vitest + Testing Library |
| `ComponentName.stories.tsx` | Storybook story |
| `component-name.module.scss` | SCSS Module, kebab-case of the component |
| `index.ts` | Barrel — the folder's public API |
| `types.ts` / `constants.ts` / `utils.ts` | Module-local |
| `useXxx.ts` | Custom hook |

Import through the `@/` alias (`@/design-system/Button`), never with `../../../` chains.

## Where does it go?

| You have… | It goes in… |
|---|---|
| A pure function used by 2+ modules | `utils/<topic>.ts` |
| A pure function used by 1 module | that module's `utils.ts` |
| A hook with no domain knowledge | `hooks/` |
| A hook that wraps a request | `api/service/<domain>.ts` |
| A hook used by one page | `pages/<Page>/hooks/` |
| A type describing an API payload | `api/<domain>.ts` |
| A generic type helper | `types/common.ts` |
| Session or scoped UI state | `contexts/<name>/` |
| App-wide UI state (notifications, theme, locale) | `stores/` |
| A component used by one page | `pages/<Page>/components/` |
| A component used by 2+ pages, domain-aware | `components/` |
| A component with no domain knowledge | `design-system/` |

## Commands

Run these from `frontend/`:

```bash
bun run check-types
bun run lint
bun run test
bun run build
bun run test:e2e
bun run storybook
bun run i18n:check
```

## Related skills

`frontend-api`, `frontend-design-system`, `frontend-component`, `frontend-page`, `frontend-form`,
`frontend-state`, `frontend-i18n`, `frontend-mocks`, `frontend-testing`.
