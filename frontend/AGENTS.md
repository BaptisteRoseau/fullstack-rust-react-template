# Frontend Codebase Architecture

> Reference for understanding, extending, and reproducing the `frontend` React app.

This document has two goals:
1. **Navigate & extend** the existing codebase quickly and to current norms.
2. **Reproduce** the same clean architecture from scratch in another project.

---

## 1. Big picture

The frontend is a **single SPA written in React + TypeScript**, bundled by **Vite**, living
entirely under `frontend/`. The architecture follows the **[bulletproof-react](https://github.com/alan2207/bulletproof-react)**
conventions: a **feature-based** layout with a strictly **unidirectional** dependency flow
(`shared → features → app`) enforced by ESLint.

### Toolchain
| Concern         | Tool |
|-----------------|------|
| Runtime / PM    | [Bun](https://bun.sh) (`bun install`, `bun run dev`) |
| Bundler         | Vite 8 (config in `vite.config.ts`) |
| Language        | TypeScript 6 (`strict`), JSX via `@vitejs/plugin-react` |
| Styling         | Tailwind CSS 4 (via `@tailwindcss/vite`) + `cva` + `tailwind-merge` |
| UI primitives   | Radix UI (headless) following the ShadCN/UI copy-in pattern; `lucide-react` icons |
| Data fetching   | TanStack Query (React Query) v5 + Axios |
| Client state    | Zustand (modals, notifications) |
| Forms           | React Hook Form + Zod (`@hookform/resolvers`) |
| Auth            | `react-query-auth` (`configureAuth`) |
| Routing         | React Router 8 (data router, `lazy` routes) |
| API mocking     | MSW (browser, tests, and standalone `mock-server.ts`) + `@mswjs/data` |
| i18n            | Lingui (`@lingui/macro`, `@lingui/react`, PO catalogs) |
| Docs            | Storybook 10 (`@storybook/react-vite`) |
| Unit/Integ tests| Vitest + Testing Library |
| E2E tests       | Playwright (against the MSW mock server) |
| Lint / Format   | ESLint (flat config) + Prettier, 4-space indent, kebab-case enforced |
| Codegen         | Plop (`bun run generate`) |

---

## 2. Source tree (most important folders)

```
frontend/
├── vite.config.ts            # Bundler + Vitest config (tailwind, lingui, react plugins)
├── tsconfig.json             # `@/* -> ./src/*` path alias, strict mode
├── eslint.config.cjs         # Flat config: import boundaries, kebab-case, a11y, tailwind
├── tailwind.config.cjs       # Tailwind theme/tokens
├── lingui.config.ts          # i18n locales + catalog paths
├── plopfile.cjs              # Component generator wiring
├── generators/component/     # Plop templates (component + stories + barrel)
├── .storybook/               # Storybook config (main.ts, preview.tsx)
├── mock-server.ts            # Standalone MSW server (used by e2e + `bun run-mock-server`)
├── e2e/                      # Playwright specs
└── src/
    ├── app/                  # ⭐ Application layer: providers, router, pages
    ├── components/           # ⭐ Shared UI components (ui/, layouts/, errors/, seo/)
    ├── features/             # ⭐ Self-contained feature modules (auth, discussions, ...)
    ├── lib/                  # ⭐ Preconfigured libraries (api-client, react-query, auth)
    ├── config/               # Global config: env vars + route paths
    ├── hooks/                # Shared reusable hooks
    ├── testing/              # Test utils, MSW mocks, data generators
    ├── i18n/                 # Lingui setup + PO catalogs (en, fr)
    ├── types/                # Shared TS types (api.ts)
    ├── utils/                # Shared pure utilities (cn, format)
    ├── assets/               # Static assets
    └── main.tsx              # Runtime bootstrap (mounts <App/> after MSW init)
```

`⭐` = the load-bearing folders; understand these first.

The **dependency rule** (ESLint `import/no-restricted-paths`): `app` may import from `features`
and shared; `features` may import from shared but **not from each other**; shared (`components`,
`hooks`, `lib`, `types`, `utils`) imports from nothing above it.

---

## 3. Folder-by-folder: layout + skeletons

### 3.1 `app/` — application layer (composition root)

The app layer is the only place allowed to wire features together. It owns providers, the router,
and the page components mapped to routes.

```
app/
├── index.tsx       # <App/> = <AppProvider><AppRouter/></AppProvider>
├── provider.tsx    # All global providers (Query, Helmet, ErrorBoundary, Auth, Notifications)
├── router.tsx      # createBrowserRouter: lazy route tree, ProtectedRoute, error boundaries
└── pages/          # Route screens (thin: compose feature components)
    ├── landing.tsx
    ├── not-found.tsx
    ├── auth/        # login.tsx, register.tsx
    └── app/         # root.tsx (protected layout outlet) + dashboard, users, profile, discussions/
```

**Provider stack (`provider.tsx`)** — every cross-cutting concern wraps the app here:
```tsx
export const AppProvider = ({ children }: { children: React.ReactNode }) => {
    const [queryClient] = React.useState(
        () => new QueryClient({ defaultOptions: queryConfig }),
    )
    return (
        <React.Suspense fallback={<Spinner size="xl" />}>
            <ErrorBoundary FallbackComponent={MainErrorFallback}>
                <HelmetProvider>
                    <QueryClientProvider client={queryClient}>
                        {import.meta.env.DEV && <ReactQueryDevtools />}
                        <Notifications />
                        <AuthLoader renderLoading={() => <Spinner size="xl" />}>
                            {children}
                        </AuthLoader>
                    </QueryClientProvider>
                </HelmetProvider>
            </ErrorBoundary>
        </React.Suspense>
    )
}
```

**Router (`router.tsx`)** — a data router built from `paths`. Routes are **code-split with `lazy`**;
a `convert()` adapter maps each module's `clientLoader`/`clientAction`/`default` to React Router's
`loader`/`action`/`Component`. The `/app` subtree is wrapped in `<ProtectedRoute>`:
```tsx
export const createAppRouter = (queryClient: QueryClient) =>
    createBrowserRouter([
        { path: paths.home.path, lazy: () => import('./pages/landing').then(convert(queryClient)) },
        {
            path: paths.app.root.path,
            element: <ProtectedRoute><AppRoot /></ProtectedRoute>,
            ErrorBoundary: AppRootErrorBoundary,
            children: [
                { path: paths.app.discussions.path, lazy: () => import('./pages/app/discussions/discussions').then(convert(queryClient)) },
                // ...
            ],
        },
        { path: '*', lazy: () => import('./pages/not-found').then(convert(queryClient)) },
    ])
```

> Pages stay **thin**: they fetch via feature API hooks and compose feature components. Business UI
> lives in `features/*/components`, not in `app/pages`.

### 3.2 `features/` — self-contained feature modules

Each feature is an isolated vertical slice. **Features must not import from one another** — compose
them at the `app` layer instead. Only the folders a feature needs are present (no empty scaffolding).

```
features/<feature>/
├── api/         # One file per endpoint: schema + fetcher + Query/Mutation hook
├── components/  # Feature-specific components (compose shared ui/)
├── hooks/       # Feature-specific hooks            (optional)
├── stores/      # Feature-specific Zustand stores   (optional)
├── types/       # Feature-specific types            (optional)
└── utils/       # Feature-specific utilities         (optional)
```

Current features: `auth`, `discussions`, `comments`, `teams`, `users`. Note that **auth is split**:
the api-call definitions + `configureAuth` live in `lib/auth.tsx` (shared across features), while
`features/auth` holds only the login/register form components.

> No barrel files for features — import the concrete file directly (`../api/create-discussion`).
> Vite tree-shakes better without re-export barrels, per the project structure doc.

### 3.3 `api/` layer (colocated in each feature)

The data-access pattern. Every endpoint declaration consists of **three parts in one file**:
1. **Zod schema + inferred input type** (for mutations / validated requests).
2. A **fetcher** using the shared `api` client.
3. A **React Query hook** (`useQuery` for reads, `useMutation` for writes) plus a
   `queryOptions` factory so loaders and components share the same key.

**Read (`get-discussions.ts`):**
```ts
export const getDiscussions = (page = 1): Promise<{ data: Discussion[]; meta: Meta }> =>
    api.get(`/discussions`, { params: { page } })

export const getDiscussionsQueryOptions = ({ page }: { page?: number } = {}) =>
    queryOptions({
        queryKey: page ? ['discussions', { page }] : ['discussions'],
        queryFn: () => getDiscussions(page),
    })

export const useDiscussions = ({ queryConfig, page }: UseDiscussionsOptions) =>
    useQuery({ ...getDiscussionsQueryOptions({ page }), ...queryConfig })
```

**Write (`create-discussion.ts`)** — schema drives both validation and the input type; the mutation
**invalidates** the matching list query on success so the cache stays fresh:
```ts
export const createDiscussionInputSchema = z.object({
    title: z.string().min(1, 'Required'),
    body: z.string().min(1, 'Required'),
})
export type CreateDiscussionInput = z.infer<typeof createDiscussionInputSchema>

export const createDiscussion = ({ data }: { data: CreateDiscussionInput }): Promise<Discussion> =>
    api.post(`/discussions`, data)

export const useCreateDiscussion = ({ mutationConfig }: UseCreateDiscussionOptions = {}) => {
    const queryClient = useQueryClient()
    const { onSuccess, ...restConfig } = mutationConfig || {}
    return useMutation({
        onSuccess: (...args) => {
            queryClient.invalidateQueries({ queryKey: getDiscussionsQueryOptions().queryKey })
            onSuccess?.(...args)
        },
        ...restConfig,
        mutationFn: createDiscussion,
    })
}
```

> Every hook accepts a `queryConfig` / `mutationConfig` passthrough (typed via `QueryConfig` /
> `MutationConfig` from `lib/react-query`) so callers can override caching/callbacks without new hooks.

### 3.4 `lib/` — preconfigured libraries

Single, app-wide configured instances of third-party libraries. **Configure once, import everywhere.**

```
lib/
├── api-client.ts     # The single Axios instance + request/response interceptors
├── react-query.ts    # queryConfig defaults + QueryConfig/MutationConfig generic helpers
├── auth.tsx          # configureAuth() → useUser/useLogin/useLogout/useRegister/AuthLoader + ProtectedRoute
└── authorization.tsx # RBAC (ROLES) + PBAC (POLICIES) + <Authorization> guard + useAuthorization
```

**`api-client.ts`** — the one HTTP client. Response interceptor unwraps `.data`, fires an error
toast via the Zustand notifications store, and redirects to login on `401`:
```ts
export const api = Axios.create({ baseURL: env.API_URL })
api.interceptors.request.use(authRequestInterceptor)      // sets Accept + withCredentials
api.interceptors.response.use(
    (response) => response.data,
    (error) => {
        const message = error.response?.data?.message || error.message
        useNotifications.getState().addNotification({ type: 'error', title: 'Error', message })
        if (error.response?.status === 401) {
            window.location.href = paths.auth.login.getHref(window.location.pathname)
        }
        return Promise.reject(error)
    },
)
```

**`auth.tsx`** — auth fetchers + `configureAuth` produce the user/login/logout/register hooks and
`<AuthLoader>`. `<ProtectedRoute>` redirects unauthenticated users to login (preserving `redirectTo`).

**`authorization.tsx`** — two-tier access control. `ROLES` (`ADMIN`/`USER`) for role checks, `POLICIES`
for granular per-resource checks. UI gates render through `<Authorization>`:
```tsx
<Authorization allowedRoles={[ROLES.ADMIN]}>…</Authorization>
<Authorization policyCheck={POLICIES['comment:delete'](user, comment)}>…</Authorization>
```
Authorization is **UX only** — always validate on the server.

### 3.5 `components/` — shared UI

App-agnostic, reusable components. Split by responsibility:

```
components/
├── ui/         # The design system: button, dialog (+ confirmation-dialog), drawer, dropdown,
│               #   form (input/textarea/select/switch/label/field-wrapper/error/form-drawer),
│               #   table (+ pagination), link, spinner, md-preview, notifications (Zustand store)
├── layouts/    # content-layout, auth-layout, dashboard-layout (+ index barrel)
├── errors/     # main.tsx (MainErrorFallback used by the root ErrorBoundary)
└── seo/        # head.tsx (react-helmet-async wrapper) + index barrel
```

**A `ui/` component directory:**
```
ui/button/
├── button.tsx          # Component (PascalCase export, kebab-case file)
├── button.stories.tsx  # Storybook stories
├── index.ts            # Barrel re-export — the public surface
└── __tests__/          # Vitest + Testing Library (where present)
```

**Component skeleton** — `cva` for variants, `cn()` (clsx + tailwind-merge) to merge classes,
`asChild` via Radix `Slot` for polymorphism:
```tsx
const buttonVariants = cva('inline-flex items-center justify-center rounded-md …', {
    variants: { variant: { default: '…', destructive: '…', outline: '…' }, size: { default: '…', sm: '…' } },
    defaultVariants: { variant: 'default', size: 'default' },
})

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> &
    VariantProps<typeof buttonVariants> & { asChild?: boolean; isLoading?: boolean; icon?: React.ReactNode }

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(({ className, variant, size, asChild, ... }, ref) => {
    const Comp = asChild ? Slot : 'button'
    return <Comp className={cn(buttonVariants({ variant, size, className }))} ref={ref} {...props}>…</Comp>
})
```

Forms compose the `ui/form` primitives. `<Form>` wires React Hook Form + a Zod schema; fields take a
`registration={register('field')}` prop and surface `formState.errors`. `<FormDrawer>` packages a
trigger button + drawer + submit for create/update flows. See `features/discussions/components/create-discussion.tsx`.

### 3.6 `config/`

- **`env.ts`** — validated, typed env vars (e.g. `API_URL`), read from `import.meta.env`.
- **`paths.ts`** — the **single source of truth for routes**. Every route has a `path` (for the
  router) and a `getHref(...)` builder (for links/redirects). Never hardcode a URL string:
```ts
export const paths = {
    home: { path: '/', getHref: () => '/' },
    auth: { login: { path: '/auth/login', getHref: (redirectTo?) => `/auth/login${redirectTo ? `?redirectTo=${encodeURIComponent(redirectTo)}` : ''}` }, … },
    app:  { discussion: { path: 'discussions/:discussionId', getHref: (id) => `/app/discussions/${id}` }, … },
} as const
```

### 3.7 `hooks/`, `utils/`, `types/`

- **`hooks/`** — shared hooks (e.g. `use-disclosure.ts` for open/close state), each with a colocated
  `__tests__/*.test.ts`. Naming: `use-kebab-case.ts`.
- **`utils/`** — pure helpers: `cn.ts` (clsx + tailwind-merge merge), `format.ts` (dayjs formatting).
- **`types/`** — shared types. `api.ts` holds the domain models (`User`, `Discussion`, `Comment`,
  `Team`, `AuthResponse`, `Meta`) returned by the API. Feature-local types stay inside the feature.

### 3.8 `testing/` — testing infrastructure

```
testing/
├── test-utils.tsx       # renderApp() wrapping UI in AppProvider + a memory router; re-exports RTL
├── setup-tests.ts       # Vitest setup (jest-dom, MSW server lifecycle)
├── data-generators.ts   # Falso-based factories (createUser, createDiscussion, …)
└── mocks/
    ├── server.ts        # MSW node server (tests)
    ├── browser.ts       # MSW browser worker (dev)
    ├── db.ts            # @mswjs/data in-memory database
    ├── utils.ts         # auth helpers (hash, authenticate, AUTH_COOKIE)
    ├── index.ts         # enableMocking() — gated on env, called from main.tsx
    └── handlers/        # Request handlers per domain (auth, users, discussions, comments, teams)
```

**`renderApp()`** seeds a user into the mock DB, logs them in via cookie, mounts the UI inside the
real `AppProvider` + a `createMemoryRouter`, and waits for loading spinners to clear:
```tsx
const { user } = await renderApp(<Discussions />, { url: '/app/discussions' })
// pass `user: null` to render unauthenticated
```
Tests hit **real HTTP through MSW** (no `fetch` mocking) — the same handlers power dev, tests, e2e,
and the standalone `mock-server.ts`. Tests live in `__tests__/` folders beside the code.

### 3.9 i18n (Lingui)

```
i18n/
├── index.ts            # i18n instance, Locale type ('en'|'fr'), defaultLocale, loadLocale()
└── locales/{en,fr}/messages.po
```
Mark strings with the Lingui macros in code: `<Trans>Create Discussion</Trans>` in JSX, `` t`Title` ``
in expressions. The `@lingui/vite-plugin` compiles catalogs. Workflow:
`bun run i18n:extract` → translate the PO files → `bun run i18n:compile` (`i18n:check` runs both `--strict`).

### 3.10 Storybook & Plop

- **Storybook** (`.storybook/main.ts`, `preview.tsx`) catalogs `ui/` components. Stories are
  `*.stories.tsx` colocated with the component (`@storybook/react-vite`, a11y + docs addons).
- **Plop** (`bun run generate`) scaffolds a component (`.tsx` + `.stories.tsx` + `index.ts` barrel)
  from `generators/component/*.hbs`, prompting for the target feature or a `components/` subfolder.

---

## 4. Conventions cheat-sheet

- **File naming:** **kebab-case** for all files and folders (`user-profile.tsx`, `use-discussions.ts`);
  **PascalCase** for component exports; `camelCase` for functions/variables. Hooks: `use-*.ts`.
- **Imports:** absolute via the **`@/` alias** (`@/components/ui/button`, `@/lib/api-client`,
  `@/config/paths`) defined in `tsconfig.json` and resolved by Vite. Relative imports only within a
  feature/component folder.
- **No cross-feature imports.** No `app` imports from shared (`features`/`app` depend on shared, never
  the reverse). Boundaries enforced by ESLint `import/no-restricted-paths`.
- **No feature barrels.** Import the concrete file (better tree-shaking); `ui/` components *do* keep an
  `index.ts` barrel as their public surface.
- **Data:** read via `useQuery`, write via `useMutation` — one file per endpoint colocating schema +
  fetcher + hook; invalidate the related query on mutation success. Never call `api` directly in a component.
- **State:** local `useState`/`useReducer` first; Zustand for global client state (notifications,
  modals); React Query for **all** server state. Avoid premature globalization.
- **Forms:** React Hook Form + Zod schema via the shared `ui/form` primitives.
- **Styling:** Tailwind utility classes only; variants via `cva`; merge via `cn()`. No CSS-in-JS.
- **Routes:** declare in `config/paths.ts` (`path` + `getHref`); register lazily in `app/router.tsx`.
- **Lint/format:** ESLint flat config + Prettier, **4-space indent**, kebab-case rule, jsx-a11y,
  tailwind ordering. Husky + lint-staged run lint + `check-types` pre-commit.

## 5. Commands

```bash
bun install              # install dependencies
bun run dev              # Vite dev server (port 3000, MSW mocks enabled)
bun run build            # tsc + vite production build
bun run test             # Vitest (unit + integration)
bun run test:e2e         # Playwright against the MSW mock server
bun run lint             # ESLint on src
bun run check-types      # tsc --noEmit
bun run format           # Prettier write
bun run storybook        # Storybook dev server (port 6006)
bun run generate         # Plop component generator
bun run i18n:extract     # extract Lingui catalog
bun run i18n:compile     # compile Lingui catalog
```
