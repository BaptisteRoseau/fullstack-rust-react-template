---
name: frontend-page
description: How to add or update a page and its route — PATHS entry, page folder, lazy route registration, layout choice, nav link and test. Use this when creating a new route, screen or view, or restructuring the router.
---

# Pages and routing

One SPA, one entry. `src/pages/` holds one folder per route, `src/router/` wires them up,
`src/layouts/` provides the chrome they render inside.

```
main.tsx → Context.tsx → App.tsx → router → layout → page
```

## Checklist

1. Add the path to `src/router/constants.ts`.
2. Scaffold the folder with `bun run generate page <PageName> "<Page title>"` — see §4.
3. Register the lazy route in `src/router/routes.tsx`, under the right layout.
4. Add a nav entry if the page is user-reachable.
5. Wrap every user-facing string in a Lingui macro, then run `bun run i18n:extract` and translate.
6. Write `<PageName>.test.tsx`.
7. Add an e2e spec in `e2e/` if the page is part of a critical journey.

## 1. Path constants

Never write a URL literal in a component. Every path and path *builder* lives here, so renaming a
route is one edit.

```ts
export const PATHS = {
    home: '/',
    login: '/auth/login',
    register: '/auth/register',
    user: {
        root: '/user',
        information: '/user',
        apiKeys: '/user/api-keys',
    },
    notFound: '*',
} as const
```

## 2. Route registration

Pages are lazily loaded so each route is its own chunk. The page barrel is what the lazy import
resolves to.

```tsx
export const router = createBrowserRouter([
    {
        HydrateFallback: () => null,
        element: <AuthLayout />,
        children: [
            { path: PATHS.login, lazy: async () => ({ Component: (await import('@/pages/Login')).Login }) },
        ],
    },
    {
        HydrateFallback: () => null,
        element: <AppLayout />,
        children: [
            { path: PATHS.home, lazy: async () => ({ Component: (await import('@/pages/Home')).Home }) },
            {
                element: <ProtectedRoute><Outlet /></ProtectedRoute>,
                children: [
                    {
                        path: PATHS.user.root,
                        lazy: async () => ({ Component: (await import('@/pages/User')).User }),
                        children: [
                            { index: true, lazy: async () => ({ Component: (await import('@/pages/User')).Information }) },
                            { path: PATHS.user.apiKeys, lazy: async () => ({ Component: (await import('@/pages/User')).ApiKeys }) },
                        ],
                    },
                ],
            },
            { path: PATHS.notFound, lazy: async () => ({ Component: (await import('@/pages/NotFound')).NotFound }) },
        ],
    },
])
```

Guard with a **pathless wrapper route** (`element: <ProtectedRoute><Outlet /></ProtectedRoute>`) so
the children keep their own `lazy`. `HydrateFallback: () => null` silences React Router's
hydration warning on lazy roots.

## 3. Layouts

- `AppLayout` — header + `<Outlet/>` + footer. The public shell.
- `AuthLayout` — centred card for `/auth/*`.
- `ContentLayout` — presentational wrapper a page renders *inside* the shell for its title,
  description and action bar.

## 4. Page folder

Generate it, never hand-write it. From `frontend/`:

```bash
bun run generate page <PageName> "<Page title>"
# e.g. bun run generate page ApiKeys "API keys"
```

That writes `src/pages/<PageName>/` with `<PageName>.tsx`, `<page-name>.module.scss`,
`<PageName>.test.tsx` and `index.ts` — steps 5 and 7 of the checklist then fill in the generated
component and test. Run it without arguments to be prompted.

The folder then grows as the page does:

```
pages/User/
├── User.tsx                # shell: left nav + <Outlet/>
├── user.module.scss
├── index.ts                # exports User, Information, ApiKeys
├── components/             # used by this page only
│   ├── UserNav/
│   ├── ApiKeysTable/
│   ├── CreateApiKeyDialog/
│   ├── RevokeApiKeyButton/
│   └── NewApiKeyBanner/
└── sections/               # one folder per child route
    ├── Information/
    └── ApiKeys/
```

Rules:

- A page folder is **private**. Nothing outside it may import from its `components/`, `hooks/` or
  `sections/`. The `index.ts` is the only surface.
- `components/` and `sections/` entries have no generator of their own — mirror the shape
  `bun run generate component` produces (`Name.tsx`, `name.module.scss`, `Name.test.tsx`,
  `index.ts`).
- When a second page needs one of those parts, move it to `src/components/` in the same commit.
- **No page imports another page** — ESLint enforces this.

## 5. Page component

A page composes; it does not implement. Data comes from `api/service/`, UI from `components/` and
`design-system/`.

```tsx
export function ApiKeys() {
    const { t } = useLingui()
    const { data, error, isLoading, mutate } = useApiKeys()
    const [isCreateOpen, setIsCreateOpen] = useState(false)

    return (
        <ContentLayout
            title={t`API keys`}
            description={t`Keys authenticate machine access to the API.`}
            actions={
                <Button onClick={() => setIsCreateOpen(true)}>
                    <PlusIcon />
                    <Trans>New key</Trans>
                </Button>
            }
        >
            <ApiKeysTable
                apiKeys={data ?? []}
                isLoading={isLoading}
                error={error}
                onRevoked={() => void mutate()}
            />
            <CreateApiKeyDialog isOpen={isCreateOpen} onOpenChange={setIsCreateOpen} />
        </ContentLayout>
    )
}
```

Pass loading and error **down** to the component that renders them, so the page body stays one
readable composition instead of a chain of early returns.

Set the document title with `<Head title={t\`…\`} />` from `@/components/head/Head`.

## 6. Sub-navigation

Prefer real nested routes over local tab state — deep links, back-button and e2e all work for free.
Use `NavLink` with a class callback for the active state, and `end` on the index link:

```tsx
<NavLink to={PATHS.user.information} end className={({ isActive }) => clsx(styles.link, isActive && styles.active)}>
```

## 7. Test

```tsx
vi.mock('@/api/service/auth')

it('renders the hero when signed out', () => {
    vi.mocked(useCurrentUser).mockReturnValue({
        data: null, error: undefined, isLoading: false, isValidating: false, mutate: vi.fn(),
    })

    render(<Home />)

    expect(
        screen.getByRole('link', { name: 'Get started' }),
        `expected a "Get started" link, got: ${document.body.textContent}`,
    ).toBeVisible()
})
```
