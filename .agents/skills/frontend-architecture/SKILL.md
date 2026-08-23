---
name: frontend-architecture
description: Use when starting frontend work, deciding where a new file belongs, or reviewing whether a change crosses a layer boundary it should not.
---

# Frontend architecture

`frontend/` is a **layer-first** React SPA: top-level folders under `src/` are technical roles, not
domains. Read this before writing any frontend code — every other `frontend-*` skill assumes it.

## 1. Know the stack

| Concern | Choice |
| --- | --- |
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

## 2. Know the directory tree

```txt
src/
├── api/            # generated SDK, domain converters, SWR hooks — Skill(frontend-api)
├── components/     # domain-aware shared components — Skill(frontend-component)
├── config/         # env.ts, validated environment variables
├── constants/      # app-wide constants that are not routes
├── css/            # tokens, mixins, reset, main.scss
├── design-system/  # domain-agnostic UI primitives — Skill(frontend-design-system)
├── hooks/          # reusable hooks with no domain knowledge
├── i18n/           # Lingui instance and PO catalogs — Skill(frontend-i18n)
├── img/            # static images and SVGs
├── layouts/        # AppLayout, AuthLayout, ContentLayout
├── pages/          # one folder per route — Skill(frontend-page)
├── router/         # routes.tsx, constants.ts
├── stores/         # Zustand stores — Skill(frontend-state)
├── test-utils/     # render helpers, MSW mocks, fixtures — Skill(frontend-testing), Skill(frontend-mocks)
├── types/          # cross-layer TypeScript types
├── utils/          # pure helpers, no JSX and no hooks
├── App.tsx         # mounts the router
├── Context.tsx     # the single provider tree
└── main.tsx        # runtime bootstrap
```

Each folder listed above has its own `README.md` describing it in more depth.

## 3. Respect the layering rules

Dependencies flow **downwards only**:

```txt
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
relax one, put the file in a different layer instead.

## 4. Name files by convention

| Pattern | Meaning |
| --- | --- |
| `ComponentName/ComponentName.tsx` | React component; folder name matches file name |
| `ComponentName.test.tsx` | Vitest + Testing Library |
| `ComponentName.stories.tsx` | Storybook story |
| `component-name.module.scss` | SCSS Module, kebab-case of the component |
| `index.ts` | Barrel — the folder's public API |
| `types.ts` / `constants.ts` / `utils.ts` | Module-local |
| `useXxx.ts` | Custom hook |

Import through the `@/` alias (`@/design-system/Button`), never with `../../../` chains.

## 5. Decide where new code goes

| You have… | It goes in… |
| --- | --- |
| A pure function used by 2+ modules | `utils/<topic>.ts` |
| A pure function used by 1 module | that module's `utils.ts` |
| A hook with no domain knowledge | `hooks/` |
| A hook that wraps a request | `api/hooks/useApiXxx/` |
| A `Promise`-returning call to the backend | `api/domains/<domain>/<domain>.ts` |
| A hook used by one page | `pages/<Page>/hooks/` |
| A type describing an API payload | `api/domains/<domain>/types.ts` — hand-written, never a generated alias |
| A generic type helper | `types/common.ts` |
| Session or scoped UI state | `contexts/<name>/` — Skill(frontend-state) |
| App-wide UI state (notifications, theme, locale) | `stores/` — Skill(frontend-state) |
| A component used by one page | `pages/<Page>/components/` |
| A component used by 2+ pages, domain-aware | `components/` — Skill(frontend-component) |
| A component with no domain knowledge | `design-system/` — Skill(frontend-design-system) |

## 6. Scaffold it — never hand-write a new folder

Every file shape in this tree has a Plop generator in `frontend/generators/`
([`generators/README.md`](../../../frontend/generators/README.md)). Run the generator, then fill in
what it produces. Do not create the folder by hand or copy an existing one — the templates are the
source of truth for the naming, the barrel and the test file.

Run from `frontend/`:

| You are adding… | Command |
| --- | --- |
| A design-system primitive | `bun run generate component design-system "" <ComponentName>` |
| …inside a grouping folder | `bun run generate component design-system inputs <ComponentName>` |
| A shared domain-aware component | `bun run generate component components <group-or-empty-string> <ComponentName>` |
| A page | `bun run generate page <PageName> "<Page title>"` |
| An API domain (types, converters, fetchers, keys, both tests, MSW handler) | `bun run generate api <domainName> <backendPath>` |
| An SWR binding over an existing domain | `bun run generate api-hook <OperationName> <domainName>` |
| A shared hook | `bun run generate hook <nameWithoutUsePrefix>` |
| A Zustand store | `bun run generate store <storeName>` |

Arguments are positional and answer the generator's prompts in order; pass `""` for an empty
grouping folder. Omit them (`bun run generate`) to be prompted interactively.

A generator only writes files — you still wire the result up: a page needs its `PATHS` entry, its
lazy route and a nav link; an API domain needs its handler registered in
`src/test-utils/mocks/handlers/index.ts`; every user-facing string needs a Lingui macro and an
extraction run.

A page-local component (`pages/<Page>/components/<Name>/`) has no generator of its own — copy the
shape the `component` generator produces.

`src/api/generated/` is the one exception in the other direction: it is written by
`./scripts/build_frontend_api_sdk.sh`, never by a person or a Plop generator — Skill(frontend-api-sdk).

## Checklist

```bash
bun run generate      # never hand-write a new component/page/domain/hook/store folder
```

- [ ] The change lives in exactly one layer — nothing reaches sideways or upwards.
- [ ] Every import crosses a layer through `@/`, never through a `../../../` chain.
