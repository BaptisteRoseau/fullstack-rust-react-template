# 07 – Bootstrap plan

← [Back to overview](README.md)

Ordered plan to replace the current feature-first (bulletproof-react) frontend with the layer-first
architecture described in these docs. Written to be executed phase by phase, each phase ending on a
green `check-types` + `lint` + `test`.

**Retained from the current setup:** Bun, Vite, Playwright, TypeScript, React 19, React Router,
Vitest, Storybook, Lingui, MSW, React Hook Form + Zod, Radix UI, Zustand.

---

## Phase 0 — Groundwork

1. Branch off `main`; the tree is rewritten in place, not migrated file by file.
2. Confirm the reference material is no longer needed at the repo root: `react-architecture/` and
   `react-architecture.zip` are untracked inputs. Delete or gitignore them once the docs here are
   accepted.
3. Read [README](README.md) and [06 – Tooling](06-tooling.md) before writing code.

---

## Phase 1 — Toolchain swap

**Remove** — React Query, Axios and the entire Tailwind stack:

```bash
bun remove @tanstack/react-query @tanstack/react-query-devtools react-query-auth axios \
           tailwindcss @tailwindcss/vite @tailwindcss/typography tailwind-merge \
           tailwindcss-animate class-variance-authority eslint-plugin-tailwindcss
```

**Add**:

```bash
bun add swr
bun add -d sass-embedded vite-plugin-svgr
```

`clsx`, `zod`, `react-hook-form`, `@hookform/resolvers`, `zustand`, `nanoid`, `dayjs` and the
Radix packages all stay.

**Config edits**:

| File | Change |
|---|---|
| `vite.config.ts` | Drop the Tailwind plugin; add `svgr()`, the `@` alias, and `css.preprocessorOptions.scss.loadPaths` ([06](06-tooling.md#making-tokens-importable)) |
| `tailwind.config.cjs` | Delete |
| `postcss.config.cjs` | Keep only if you want `autoprefixer`; otherwise delete with its dep |
| `eslint.config.cjs` | Remove the Tailwind plugin; switch `check-file` to PascalCase component folders; add the two `no-restricted-imports` layer rules ([06](06-tooling.md#linting)) |
| `tsconfig.json` | Ensure `paths` maps `@/*` → `src/*` |
| `src/index.css` | Delete — replaced by `src/css/` |

**Gate:** `bun run check-types` passes on an empty-ish `src/` — expect failures only from files
scheduled for deletion in Phase 2.

---

## Phase 2 — Wipe and scaffold

Delete `src/` except `config/env.ts`, `i18n/locales/*.po` and `assets/`, then create the skeleton:

```
src/{api/{service/__mocks__,utils},components,config,constants,contexts,css,design-system,
     hooks,i18n,img,layouts,pages,router,stores,stories,test-utils/{mocks/handlers,fixtures},
     types,utils}
```

Write, in this order — each depends on the previous:

1. `src/css/` — `_variables.scss`, `_themes.scss`, `_mixins.scss`, `_reset.scss`, `main.scss`
   ([06](06-tooling.md#global-styles-and-tokens))
2. `src/types/declarations.d.ts` — **do this before any component**, or every `.module.scss`
   import is a type error
3. `src/api/generated/` — run `./scripts/build_frontend_api_sdk.sh` ([01](01-api.md))
4. `src/api/errors.ts`, `src/api/client.ts` ([01](01-api.md))
5. `src/utils/createContext.tsx`, `src/utils/assert.ts`
6. `src/router/constants.ts` — the `PATHS` object
7. `src/Context.tsx`, `src/App.tsx`, `src/main.tsx` ([04](04-pages-router.md#bootstrap-chain))

**Gate:** `bun run dev` serves a blank page with no console errors.

---

## Phase 3 — Design system

Build primitives bottom-up. Each one is a folder with component, SCSS module, story, test and
barrel — the story is not optional ([02](02-design-system.md)).

Suggested order (later ones consume earlier ones):

1. `Icon` + `makeIcon`, `Spinner`
2. `Button`, `IconButton`, `Link`
3. `inputs/`: `TextInput`, `TextArea`, `SelectInput`, `CheckboxInput`, `SwitchInput`
4. `Badge`, `Tag`, `Avatar`, `Card`
5. Radix wrappers: `Dialog`, `Drawer`, `Dropdown`, `Tabs`, `Tooltip`
6. `Table`, `Pagination`, `ProgressBar`

Port the visual design from the current Tailwind components by reading their utility strings and
translating them into the token scale — do not invent a new look while restructuring.

**Gate:** `bun run storybook` renders every primitive in light and dark; `bun run test` green.

---

## Phase 4 — API layer

One domain at a time, mirroring the backend resources in `crates/api`. For each:

1. `src/api/domains/<domain>/types.ts` — hand-written domain types
2. `src/api/domains/<domain>/converters.ts` + `converters.test.ts` — wire types in, domain types out
3. `src/api/domains/<domain>/keys.ts`, `<domain>.ts`, `<domain>.test.ts`, `index.ts`
4. `src/api/hooks/useApiXxx/` — one folder per operation, with its test
5. `src/test-utils/mocks/handlers/<domain>.ts` + `src/test-utils/fixtures/<domain>.ts`

Rebuild the MSW in-memory DB (`test-utils/mocks/db.ts`) and `mock-server.ts` against the new
handler layout as you go — the dev server and e2e both depend on them.

**Gate:** `bun run run-mock-server` responds on every migrated endpoint; domain and hook tests green.

---

## Phase 5 — Shared components

Build `src/components/` in dependency order ([03](03-components.md)):

1. `errors/ErrorFallback` (needed by `Context.tsx`), `head/Head`
2. `forms/`: `Form`, `FormField`, then the `fields/*` — this unblocks every page with a form
3. `notifications/` + `src/stores/notifications.ts`
4. `layout/`: `AppHeader`, `AppSidebar`, `UserMenu`
5. `ProtectedRoute`, `ConfirmationDialog`, `DataTable`, `MarkdownPreview`
6. `src/layouts/`: `AppLayout`, `AuthLayout`, `ContentLayout`

**Gate:** layouts render inside Storybook with mocked services.

---

## Phase 6 — Pages and routing

For each page: folder under `src/pages/`, entry in `PATHS`, lazy route in `routes.tsx`, nav link,
test ([04](04-pages-router.md#adding-a-page--checklist)).

Order: `Login`, `Register`, `NotFound`, then the authenticated pages. Add loaders only where a real
waterfall shows up.

**Gate:** every route reachable by clicking; `bun run build` succeeds.

---

## Phase 7 — Tests, i18n, e2e

1. `src/test-utils/render.tsx`, `renderAppAtRoute.tsx`, `wrappers.tsx`, `setup-tests.ts`
2. Backfill component and page tests to match the levels table in
   [06](06-tooling.md#testing)
3. `bun run i18n:extract && bun run i18n:compile` — the catalogs survive the rewrite, but keys move
4. Rewrite `e2e/` specs against the new routes and accessible names

**Gate:** `bun run test`, `bun run test:e2e`, `bun run i18n:check` all green.

---

## Phase 8 — Documentation and tooling cleanup

**This phase is not optional.** A large amount of in-repo documentation describes the *old*
architecture and becomes actively misleading — it will send both humans and agents to
`src/features/`, React Query and Tailwind on day one.

| Artefact | Action |
|---|---|
| `frontend/docs/*.md` (13 flat files: `api-layer.md`, `project-structure.md`, `components-and-styling.md`, `state-management.md`, `testing.md`, …) | Delete — superseded by `docs/architecture/` |
| `frontend/AGENTS.md` (~20 KB, documents bulletproof-react) | Rewrite against the new structure |
| `frontend/src/README.md` and every per-folder `README.md` | Rewrite; the folder set has changed entirely |
| `.claude/skills/frontend-*` — **15 skills** (`frontend-react-architecture`, `-api`, `-component`, `-feature`, `-form`, `-hook`, `-layout`, `-mocks`, `-page`, `-state`, `-testing`, `-authorization`, `-i18n`, `frontend-new-page`, `frontend-storybook`) | Rewrite. `frontend-react-feature` has no target left and should be deleted; the rest need new paths, SWR instead of React Query, SCSS instead of Tailwind |
| `frontend/generators/` (Plop templates: `api`, `component`, `feature`, `form`, `hook`, `layout`, `page`, `store`) + `plopfile.cjs` | Rewrite templates for the new folder shapes; drop the `feature` generator |
| `frontend/README.md` | Update the architecture section |
| `.cspell.json` | Add new terms; remove dead ones |

**Gate:** grep the repo for `features/`, `react-query`, `tailwind`, `cva`, `bulletproof` — no
stale hits outside a deliberate changelog.

---

## Phase 9 — Final verification

```bash
bun run check-types
bun run lint
bun run test
bun run build
bun run test:e2e
bun run storybook:build
bun run i18n:check
```

Then confirm against the layering rules: no `design-system/` file imports `api/` or `contexts/`;
no `pages/*` imports another page; no component imports `api/client.ts` directly.

---

## Effort and sequencing notes

- **Phases 1–2 are a hard cut.** The app does not build in the middle of them; do not try to keep
  it running.
- **Phases 3–6 are incremental** and individually reviewable. Phase 3 is the bulk of the work —
  roughly 25 primitives to restyle in SCSS.
- **Phase 8 is where this kind of rewrite usually rots.** The 15 skills and the Plop generators are
  what future contributors and agents actually read; leaving them stale silently reintroduces the
  old architecture one file at a time.
- Phases 4 and 3 can proceed in parallel if two people are working — they only meet in Phase 5.
