# `app/`

The **application layer** — the composition root. The only place allowed to import from `features`
and wire them together.

| File / folder | Role |
|---------------|------|
| `index.tsx` | `<App/>` = `<AppProvider><AppRouter/></AppProvider>`. |
| `provider.tsx` | All global providers: `Suspense → ErrorBoundary → HelmetProvider → QueryClientProvider → Notifications + AuthLoader`. Add new app-wide providers **here**. |
| `router.tsx` | `createBrowserRouter` with lazy, code-split routes; `convert()` maps each page's `clientLoader`/`clientAction`/`default` to React Router. `/app` subtree is wrapped in `ProtectedRoute`. |
| `pages/` | Route screens. Thin: compose feature components inside a layout. |

## Adding a route

1. Add the path to `config/paths.ts`.
2. Create a `default`-exported page under `pages/**` (wrap in `ContentLayout`).
3. Register a lazy entry in `router.tsx` (under the `/app` children if authenticated).
4. Add a sidebar link in `components/layouts/dashboard-layout.tsx` if needed.

See `.claude/skills/frontend-react-page`.

## Pages

- `pages/landing.tsx`, `pages/not-found.tsx` — public.
- `pages/auth/` — login, register.
- `pages/app/` — authenticated screens (`root.tsx` is the `DashboardLayout` outlet parent) + `discussions/`, `users.tsx`, `profile.tsx`, `dashboard.tsx`.

Pages may export `clientLoader`/`clientAction` factories to prefetch via React Query before render.
