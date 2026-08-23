# Frontend Architecture

Canonical description of the React frontend's structure, layering rules and file conventions.
This set supersedes the flat `frontend/docs/*.md` files, which document the previous architecture.

## Table of contents

1. [What this architecture is called](#what-this-architecture-is-called)
2. [Stack](#stack)
3. [Directory tree](#directory-tree)
4. [Layering rules](#layering-rules)
5. [File-naming conventions](#file-naming-conventions)
6. Sub-sections
   - [01 – API layer](01-api.md)
   - [02 – Design system](02-design-system.md)
   - [03 – Shared components](03-components.md)
   - [04 – Pages & router](04-pages-router.md)
   - [05 – Hooks, utils & types](05-hooks-utils-types.md)
   - [06 – Tooling: styles, state, i18n, tests](06-tooling.md)
   - [07 – Bootstrap plan](07-bootstrap-plan.md)

---

## What this architecture is called

This is a **layer-first** (also called *type-based*, *horizontally sliced*, or *group-by-file-type*)
architecture. Top-level folders are technical roles — `api/`, `components/`, `pages/`, `hooks/`,
`utils/`, `types/` — and a given domain's code is spread across them.

It combines four well-known patterns:

| Pattern | Where it shows up here |
|---|---|
| **Colocation** (test, story and stylesheet live next to the source) | Every component folder |
| **Barrel / module public API** (`index.ts` re-exports; importers never reach inside) | Every component and multi-file module |
| **Two-tier UI split** — domain-agnostic primitives vs. domain-aware composites | `design-system/` vs. `components/` |
| **Generated client + anti-corruption layer** (wire types converted at the boundary) | `api/generated/` vs. `api/domains/<domain>/` vs. `api/hooks/` |

The two-tier UI split is essentially the *atoms* boundary of **Atomic Design**, and matches the
classic **presentational vs. container** distinction: nothing in `design-system/` may import an API
service or an application context.

> **This replaces a feature-first architecture.** The previous layout was
> [bulletproof-react](https://github.com/alan2207/bulletproof-react) — vertical slices under
> `src/features/<domain>/{api,components,hooks,types}` with an ESLint-enforced
> `shared → features → app` dependency rule.
>
> Know what you give up: feature-first makes a domain's boundary explicit and mechanically
> enforceable. Layer-first does not — nothing stops `pages/Catalog` from importing
> `pages/Settings`. The discipline in [Layering rules](#layering-rules) is a convention backed by
> review, not by the linter. In exchange you get one obvious home for every file, a single
> design-system boundary, and no debate about which feature a shared thing belongs to.

---

## Stack

| Concern | Choice | Notes |
|---|---|---|
| Package manager | **Bun** | unchanged |
| Bundler / dev server | **Vite** | single SPA entry, no multi-entry setup |
| Language | **TypeScript** | strict; no `any`, no `@ts-expect-error` |
| Routing | **React Router** | `createBrowserRouter`, route objects in `src/router/` |
| Server state | **SWR** | one binding per operation under `api/hooks/`, see [01](01-api.md) |
| HTTP transport | **generated SDK** over `fetch` | `@hey-api/openapi-ts` from the backend's OpenAPI document; no Axios |
| Global client state | **Zustand** | only for genuinely app-wide UI state (notifications, modals) |
| Scoped state | **React context** | one folder per context under `src/contexts/` |
| Styling | **SCSS Modules** + design tokens | `*.module.scss`, global tokens in `src/css/` |
| Accessible behaviour | **Radix UI** | unstyled primitives under the design system; we own the CSS |
| Class composition | **`clsx`** | replaces `cva` / `tailwind-merge` |
| Forms | **React Hook Form + Zod** | retained; not part of the adopted reference |
| i18n | **Lingui** | Vite plugin (SWC), no Babel |
| Unit / integration tests | **Vitest + Testing Library** | shares the Vite config |
| API faking | automocked hooks (unit) + **MSW** (integration, dev server, e2e) | see [06](06-tooling.md) |
| E2E | **Playwright** | unchanged |
| Component catalogue | **Storybook** | stories colocated with components |

---

## Directory tree

```
frontend/
├── e2e/                        # Playwright specs, one file per user journey
│   ├── <journey>.spec.ts
│   └── utils/
│       └── a11yCheck.ts
├── public/
├── src/
│   ├── api/                    # ← 01: generated SDK, domain layer, SWR hooks
│   ├── components/             # ← 03: domain-aware shared components
│   ├── config/                  # env.ts — validated environment variables
│   ├── constants/              # App-wide constants
│   ├── contexts/               # React contexts (one folder each)
│   ├── css/                    # Global SCSS: tokens, mixins, reset
│   ├── design-system/          # ← 02: domain-agnostic UI primitives
│   ├── hooks/                  # ← 05: shared hooks
│   ├── i18n/                   # Lingui setup + PO catalogs
│   ├── img/                    # Static images and SVGs
│   ├── layouts/                # Page shells (AppLayout, AuthLayout)
│   ├── pages/                  # ← 04: one folder per route
│   ├── router/                 # ← 04: route objects, loaders, path constants
│   ├── stores/                 # Zustand stores (app-wide UI state only)
│   ├── stories/                # Storybook global config & decorators
│   ├── test-utils/             # ← 06: shared render helpers and fixtures
│   ├── types/                  # ← 05: shared TypeScript types
│   ├── utils/                  # ← 05: pure helpers
│   ├── App.tsx                 # Mounts the router
│   ├── Context.tsx             # Provider tree (SWR, i18n, error boundary)
│   └── main.tsx                # Runtime bootstrap: starts MSW, renders <App/>
├── index.html
├── playwright.config.ts
├── vite.config.ts
└── tsconfig.json
```

### Deliberate omissions from the reference architecture

| Reference had | Here | Why |
|---|---|---|
| `applications/frontoffice/` vs. `pages/` | one `src/pages/` | Single SPA — there is no public/admin split |
| `src/entries/*` (many Vite entries) | one `src/main.tsx` | Not a multi-page server-rendered app |
| `babel/i18n/`, `vite/plugins/entryLocale/` | Lingui Vite plugin | No Babel in this toolchain |
| `src/redux/` | Zustand store | Redux existed only as legacy in the reference |
| `src/deprecatedApi/` | — | Nothing to deprecate in a fresh build |
| Design system named after a product | `src/design-system/` | Product-neutral naming |

---

## Layering rules

Dependencies flow **downwards only**. A module may import from any layer below it, never above.

```
              pages/  ──────────────┐
                 │                  │
            components/  ───────────┤
                 │                  │
         design-system/             │
                 │                  │
   hooks/ · utils/ · types/ · css/  │
                 │                  │
         api/hooks/  ◄──────────────┤
                 │                  │
    api/domains/<domain>/           │
                 │                  │
        api/generated/  ◄───────────┘
```

Inside `src/api/` the same rule applies downwards: `hooks/` binds SWR to a domain's fetchers, a
domain converts wire types to its own, and only `api/domains/<domain>/converters.ts` and
`api/client.ts` may name anything from `api/generated/`. ESLint enforces both boundaries.

- **`design-system/`** may import `utils/`, `types/`, `css/`, `hooks/`, Radix. It may **never**
  import `api/`, `contexts/`, or anything from `components/` or `pages/`. If a primitive needs data,
  it takes it as a prop.
- **`components/`** may import `design-system/`, `api/domains/<domain>/`, `api/hooks/`, `contexts/`,
  `hooks/`. Domain awareness is exactly what distinguishes it from the design system.
- **`pages/`** may import everything. Pages are the only place routing concerns appear.
- **`api/`** imports nothing from the UI layers.
- **Page-to-page imports are forbidden.** Shared UI moves up into `components/`.

---

## File-naming conventions

Components use `PascalCase` for folders and `.tsx` files; everything else is lowercase.

| Pattern | Meaning |
|---|---|
| `ComponentName/ComponentName.tsx` | React component (folder name matches file name) |
| `ComponentName.test.tsx` | Unit / integration test (Vitest + Testing Library) |
| `ComponentName.stories.tsx` | Storybook story |
| `component-name.module.scss` | SCSS Module — kebab-case of the component name |
| `index.ts` | Barrel export; the folder's public API |
| `types.ts` | Types local to the module |
| `constants.ts` | Module-level constants |
| `utils.ts` | Module-level pure helpers |
| `useXxx.ts` / `hooks.ts` | Custom hooks |
| `XxxContext.ts` / `XxxContextProvider.tsx` | Context definition / provider |
| `__mocks__/<name>.ts` | Manual module mock resolved by `vi.mock` |
| `__snapshots__/` | Vitest snapshots (design-system primitives only) |

Import via the `@/` alias — `import { Button } from '@/design-system/Button'` — never with deep
relative chains (`../../../`). The alias resolves in TypeScript, Vite and SCSS alike.
