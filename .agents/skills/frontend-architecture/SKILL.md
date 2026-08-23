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
| Server state | SWR — one binding per operation under `api/hooks/`, no global `fetcher` |
| HTTP | SDK generated from the backend's OpenAPI document, over `fetch` — no Axios |
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
├── api/            # generated SDK, domain converters, SWR hooks
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
                    ↘ api/hooks/ → api/domains/<domain>/ → api/generated/
```

- `design-system/` may import `utils/`, `types/`, `css/`, `hooks/`, Radix. It may **never** import
  `api/`, `contexts/`, `components/`, `pages/` or `layouts/`. A primitive takes data as props.
- `components/` may import `design-system/`, `api/domains/<domain>/`, `api/hooks/`, `contexts/`,
  `hooks/`. Domain awareness is exactly what distinguishes it from the design system.
- `pages/` may import everything. Routing concerns appear only here and in `router/`.
- `api/` imports nothing from the UI layers, and layers internally too: only
  `api/domains/<domain>/converters.ts` and `api/client.ts` may name anything from `api/generated/`,
  and outside `src/api/**` a domain is reachable only through its barrel.
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
| A hook that wraps a request | `api/hooks/useApiXxx/` |
| A `Promise`-returning call to the backend | `api/domains/<domain>/<domain>.ts` |
| A hook used by one page | `pages/<Page>/hooks/` |
| A type describing an API payload | `api/domains/<domain>/types.ts` — hand-written, never a generated alias |
| A generic type helper | `types/common.ts` |
| Session or scoped UI state | `contexts/<name>/` |
| App-wide UI state (notifications, theme, locale) | `stores/` |
| A component used by one page | `pages/<Page>/components/` |
| A component used by 2+ pages, domain-aware | `components/` |
| A component with no domain knowledge | `design-system/` |

## Scaffolding — never hand-write a new folder

Every file shape described in these skills has a Plop generator in `frontend/generators/`. Run the
generator, then fill in the generated files. Do not create the folder or copy an existing one by
hand — the templates are the source of truth for the naming, the barrel and the test file.

`src/api/generated/` is the exception in the other direction: it is written by
`./scripts/build_frontend_api_sdk.sh`, never by a person. See the `frontend-api-sdk` skill.

Run from `frontend/`:

| You are adding… | Command |
|---|---|
| A design-system primitive | `bun run generate component design-system "" <ComponentName>` |
| …inside a grouping folder | `bun run generate component design-system inputs <ComponentName>` |
| A shared domain-aware component | `bun run generate component components <group-or-empty-string> <ComponentName>` |
| A page | `bun run generate page <PageName> "<Page title>"` |
| An API domain (declaration, service, mock, test, MSW handler) | `bun run generate api <domainName> <endpointPath>` |
| A shared hook | `bun run generate hook <nameWithoutUsePrefix>` |
| A Zustand store | `bun run generate store <storeName>` |

Arguments are positional and answer the generator's prompts in order; pass `""` for an empty
grouping folder. Omit the arguments (`bun run generate`) to be prompted interactively.

A generator only writes files — you still wire the result up: a page needs its `PATHS` entry, its
lazy route and a nav link; an API domain needs its handler registered in
`src/test-utils/mocks/handlers/index.ts`; every user-facing string needs a Lingui macro and an
extraction run.

A page-local component (`pages/<Page>/components/<Name>/`) has no generator — copy the shape the
`component` generator produces.

## Commands

Run these from `frontend/`:

```bash
bun run generate      # Plop scaffolding, see above
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
`frontend-state`, `frontend-i18n`, `frontend-mocks`, `frontend-testing`, `frontend-seo`.
