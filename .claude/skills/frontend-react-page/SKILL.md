---
name: frontend-react-page
description: How to add or update a page/route — paths.ts entry, page component, lazy route registration, optional data loader, and nav link. Use this when creating or updating a frontend page, route, screen, or view.
---

# Adding a Page / Route

A route requires three edits: a path in `config/paths.ts`, a page component in `src/app/pages/**`,
and a lazy entry in `app/router.tsx`. Authenticated screens also use `ContentLayout` and (often) the
sidebar nav.

## 1. Declare the path — `src/config/paths.ts`

Single source of truth. Every route has `path` (router) + `getHref` (links/redirects):
```ts
app: {
    // ...
    thing: { path: 'things/:thingId', getHref: (id: string) => `/app/things/${id}` },
}
```
Never hardcode a URL elsewhere — always `paths.app.thing.getHref(id)`.

## 2. Page component — `src/app/pages/app/<name>.tsx`

Pages are **thin**: wrap in a layout and compose feature components. Export `default`.

```tsx
import { ContentLayout } from '@/components/layouts'
import { Things } from '@/features/things/components/things'

const ThingsRoute = () => {
    return (
        <ContentLayout title="Things">
            <Things />
        </ContentLayout>
    )
}

export default ThingsRoute
```

`ContentLayout` already wraps content in the `container` class (centered, responsive, `px` from
`tailwind.config.cjs`) — do **not** add manual `mx-auto max-w-7xl`. Standalone pages (landing, 404)
use `<div className="container py-12">` directly.

### Optional: prefetch with a data loader

Export `clientLoader` (and/or `clientAction`); `router.tsx`'s `convert()` maps them to React Router:
```tsx
import { QueryClient } from '@tanstack/react-query'
import { getThingsQueryOptions } from '@/features/things/api/get-things'

export const clientLoader = (queryClient: QueryClient) => async () => {
    const query = getThingsQueryOptions()
    return queryClient.getQueryData(query.queryKey) ?? (await queryClient.fetchQuery(query))
}
```

## 3. Register the route — `src/app/router.tsx`

Authenticated pages go in the `paths.app.root.path` children array (already inside `<ProtectedRoute>`):
```ts
{ path: paths.app.thing.path, lazy: () => import('./pages/app/thing').then(convert(queryClient)) },
```
Top-level/public pages go at the router root array. All routes use `lazy` for code-splitting.

## 4. Sidebar nav (if user-facing) — `src/components/layouts/dashboard-layout.tsx`

Add to the `navigation` array (uses Lingui `t` + a `lucide-react` icon):
```ts
import { Box } from 'lucide-react'
// ...
{ name: t`Things`, to: paths.app.thing.getHref(), icon: Box },
```

## Checklist

- [ ] Path in `paths.ts` with `getHref`
- [ ] `default`-exported page under `src/app/pages/**`, wrapped in a layout
- [ ] Lazy route in `router.tsx` (under `/app` children if authenticated)
- [ ] Nav link if it belongs in the sidebar
- [ ] Strings wrapped in Lingui macros (`frontend-react-i18n`)
