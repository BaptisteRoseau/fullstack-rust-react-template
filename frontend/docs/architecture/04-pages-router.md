# 04 – Pages & router

← [Back to overview](README.md)

One SPA, one entry. `src/pages/` holds one folder per route; `src/router/` wires them up;
`src/layouts/` provides the chrome they render inside.

---

## Bootstrap chain

```
main.tsx  →  Context.tsx  →  App.tsx  →  router  →  layout  →  page
```

```tsx
// src/main.tsx
import { createRoot } from 'react-dom/client';

import { App } from './App';
import { enableMocking } from './test-utils/enableMocking';
import './css/main.scss';

// MSW must be running before the first render, or the initial fetches escape it.
void enableMocking().then(() => {
    createRoot(document.getElementById('root')!).render(<App />);
});
```

```tsx
// src/App.tsx
import { RouterProvider } from 'react-router';

import { Context } from './Context';
import { router } from './router/routes';

export function App() {
    return (
        <Context>
            <RouterProvider router={router} />
        </Context>
    );
}
```

```tsx
// src/Context.tsx — the single provider tree
import { I18nProvider } from '@lingui/react';
import { ErrorBoundary } from 'react-error-boundary';
import { SWRConfig } from 'swr';

import { ErrorFallback } from '@/components/errors/ErrorFallback';
import { Notifications } from '@/components/notifications/Notifications';
import { i18n } from '@/i18n';

export function Context({ children }: { children: React.ReactNode }) {
    return (
        <ErrorBoundary FallbackComponent={ErrorFallback}>
            <I18nProvider i18n={i18n}>
                <SWRConfig
                    value={{
                        revalidateOnFocus: false,
                        shouldRetryOnError: false,
                    }}
                >
                    {children}
                    <Notifications />
                </SWRConfig>
            </I18nProvider>
        </ErrorBoundary>
    );
}
```

`Context.tsx` is also what `test-utils/render` reuses, so tests and the app share one provider
tree. Add a provider in one place only.

---

## Router (`src/router/`)

```
src/router/
├── routes.tsx          # createBrowserRouter route objects
├── constants.ts        # PATHS — the single source of truth for URLs
├── loaders.ts          # Route loaders (SWR prefetch / guards)
└── types.ts            # Route param and handle types
```

### Path constants

Never write a URL literal in a component. Every path — and every path *builder* — lives here, so
renaming a route is one edit.

```ts
// src/router/constants.ts
export const PATHS = {
    home: '/',
    login: '/auth/login',
    register: '/auth/register',
    users: {
        list: '/users',
        detail: (userId: string) => `/users/${userId}`,
    },
    settings: '/settings',
    notFound: '*',
} as const;
```

```tsx
<Link to={PATHS.users.detail(user.id)}>{user.email}</Link>
```

### Route definitions

Pages are lazily loaded so each route is its own chunk. The barrel export of the page folder is
what the lazy import resolves to.

```tsx
// src/router/routes.tsx
import { createBrowserRouter } from 'react-router';

import { AppLayout } from '@/layouts/AppLayout';
import { AuthLayout } from '@/layouts/AuthLayout';
import { ProtectedRoute } from '@/components/ProtectedRoute';
import { PATHS } from './constants';
import { userLoader } from './loaders';

export const router = createBrowserRouter([
    {
        element: <AuthLayout />,
        children: [
            { path: PATHS.login, lazy: async () => ({ Component: (await import('@/pages/Login')).Login }) },
            { path: PATHS.register, lazy: async () => ({ Component: (await import('@/pages/Register')).Register }) },
        ],
    },
    {
        element: (
            <ProtectedRoute>
                <AppLayout />
            </ProtectedRoute>
        ),
        children: [
            { path: PATHS.home, lazy: async () => ({ Component: (await import('@/pages/Dashboard')).Dashboard }) },
            { path: PATHS.users.list, lazy: async () => ({ Component: (await import('@/pages/Users')).Users }) },
            {
                path: PATHS.users.detail(':userId'),
                loader: userLoader,
                lazy: async () => ({ Component: (await import('@/pages/UserDetail')).UserDetail }),
            },
            { path: PATHS.settings, lazy: async () => ({ Component: (await import('@/pages/Settings')).Settings }) },
        ],
    },
    { path: PATHS.notFound, lazy: async () => ({ Component: (await import('@/pages/NotFound')).NotFound }) },
]);
```

### Loaders

Loaders exist to remove request waterfalls, not to replace hooks. Prime the SWR cache with
`preload`, then let the page's own hook read it synchronously.

```ts
// src/router/loaders.ts
import { preload } from 'swr';
import type { LoaderFunctionArgs } from 'react-router';

import { fetchUser, userKeys } from '@/api/domains/users';

export async function userLoader({ params }: LoaderFunctionArgs) {
    const { userId } = params;
    if (!userId) {
        throw new Response('Missing userId', { status: 400 });
    }

    void preload(userKeys.detail(userId), ([, id]) => fetchUser(id));
    return null;
}
```

Add a loader only when a measurable waterfall exists. A page whose data loads fast enough with a
plain hook does not need one.

---

## Layouts (`src/layouts/`)

```
src/layouts/
├── AppLayout/
│   ├── AppLayout.tsx           # header + sidebar + <Outlet/>
│   ├── app-layout.module.scss
│   └── index.ts
├── AuthLayout/                 # centred card for login/register
└── ContentLayout/              # in-page title + actions wrapper
```

```tsx
// src/layouts/AppLayout/AppLayout.tsx
import { Outlet } from 'react-router';

import { AppHeader } from '@/components/layout/AppHeader';
import { AppSidebar } from '@/components/layout/AppSidebar';

import styles from './app-layout.module.scss';

export function AppLayout() {
    return (
        <div className={styles.layout}>
            <AppHeader />
            <AppSidebar />
            <main className={styles.content}>
                <Outlet />
            </main>
        </div>
    );
}
```

`AppLayout` is the shell (routing-aware, one per route group). `ContentLayout` is a presentational
wrapper a page renders *inside* it for its title and action bar.

---

## Pages (`src/pages/`)

```
src/pages/
├── Dashboard/
│   ├── Dashboard.tsx
│   ├── Dashboard.test.tsx
│   ├── dashboard.module.scss
│   └── index.ts
│
├── Users/                          # page with local parts
│   ├── Users.tsx
│   ├── Users.test.tsx
│   ├── users.module.scss
│   ├── index.ts
│   ├── components/                 # used by this page only
│   │   ├── UsersTable/
│   │   ├── CreateUserDrawer/
│   │   └── UserFilters/
│   ├── hooks/
│   │   └── useUserFilters.ts
│   ├── constants.ts
│   └── types.ts
│
├── UserDetail/
│   ├── UserDetail.tsx
│   ├── index.ts
│   └── tabs/                       # one file per tab
│       ├── ProfileTab.tsx
│       ├── ApiKeysTab.tsx
│       └── ActivityTab.tsx
│
├── Settings/
├── Login/
├── Register/
└── NotFound/
```

Rules:

- A page folder is **private**. Nothing outside it may import from its `components/` or `hooks/`.
  The `index.ts` exports the page component and nothing else.
- When a second page needs one of those parts, move it to `src/components/` in the same commit.
- **No page imports another page.**

### Page component

A page composes; it does not implement. Data comes from `api/hooks/`, UI from `components/` and
`design-system/`.

```tsx
// src/pages/Users/Users.tsx
import { Trans, useLingui } from '@lingui/react/macro';

import { useApiUsers } from '@/api/hooks/useApiUsers';
import { ContentLayout } from '@/layouts/ContentLayout';
import { Button } from '@/design-system/Button';
import { useBooleanState } from '@/hooks/useBooleanState';

import { CreateUserDrawer } from './components/CreateUserDrawer';
import { UsersTable } from './components/UsersTable';

export function Users() {
    const { t } = useLingui();
    const { data, error, isLoading } = useUsers();
    const { value: isDrawerOpen, setTrue: openDrawer, setFalse: closeDrawer } = useBooleanState();

    return (
        <ContentLayout
            title={t`Users`}
            actions={
                <Button onClick={openDrawer}>
                    <Trans>Add user</Trans>
                </Button>
            }
        >
            <UsersTable users={data?.results ?? []} isLoading={isLoading} error={error} />
            <CreateUserDrawer isOpen={isDrawerOpen} onClose={closeDrawer} />
        </ContentLayout>
    );
}
```

Loading and error states are passed down to the component that renders them, so the page body stays
a single readable composition instead of a chain of early returns.

### Tabs

Each tab is its own file under `tabs/`, never a branch inside the page component.

```tsx
// src/pages/UserDetail/UserDetail.tsx
<Tabs defaultValue="profile">
    <TabsList>
        <TabsTrigger value="profile"><Trans>Profile</Trans></TabsTrigger>
        <TabsTrigger value="api-keys"><Trans>API keys</Trans></TabsTrigger>
    </TabsList>
    <TabsContent value="profile"><ProfileTab userId={userId} /></TabsContent>
    <TabsContent value="api-keys"><ApiKeysTab userId={userId} /></TabsContent>
</Tabs>
```

### Multi-step flows

Wizards share state through a context + reducer scoped to the page folder, never through global
state:

```
pages/Onboarding/
├── Onboarding.tsx
├── context/
│   ├── OnboardingContext.ts
│   ├── OnboardingProvider.tsx
│   ├── reducer.ts
│   └── index.ts
└── steps/
    ├── AccountStep/
    ├── WorkspaceStep/
    └── ConfirmStep/
```

---

## Adding a page — checklist

1. Add the path to `src/router/constants.ts`.
2. Create `src/pages/<PageName>/` with `<PageName>.tsx` and `index.ts`.
3. Register the lazy route in `src/router/routes.tsx`, under the right layout.
4. Add the nav entry in `src/components/layout/AppSidebar` if it is user-reachable.
5. Wrap user-facing strings in Lingui macros and run `bun run i18n:extract`.
6. Write `<PageName>.test.tsx` rendering at the route via `renderAppAtRoute`.
7. Add an e2e spec in `e2e/` if the page is part of a critical journey.
