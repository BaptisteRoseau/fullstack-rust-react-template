---
name: frontend-react-layout
description: How to create or update a page-shell layout under src/components/layouts (ContentLayout, AuthLayout, DashboardLayout). Use this when building or editing a frontend layout, page shell, sidebar, or content wrapper.
---

# Layouts

Page shells live in `src/components/layouts/` and are re-exported from its `index.ts` barrel.
Existing: `ContentLayout`, `AuthLayout`, `DashboardLayout`.

- **`DashboardLayout`** — the authenticated shell (sidebar + top bar). Used by `app/pages/app/root.tsx`, the `<Outlet>` parent for all `/app` routes.
- **`ContentLayout`** — per-page wrapper: sets the `<Head>` title and renders the page heading + body inside the `container`. Used by every app page.
- **`AuthLayout`** — the login/register shell.

## ContentLayout pattern

```tsx
import * as React from 'react'

import { Head } from '../seo'

type ContentLayoutProps = { children: React.ReactNode; title: string }

export const ContentLayout = ({ children, title }: ContentLayoutProps) => {
    return (
        <>
            <Head title={title} />
            <div className="py-6">
                <div className="container">
                    <h1 className="text-2xl font-semibold text-gray-900">{title}</h1>
                </div>
                <div className="container py-6">{children}</div>
            </div>
        </>
    )
}
```

## Rules

- **The `container` class does the centering.** It's configured in `tailwind.config.cjs` (`center: true`, responsive max-width, horizontal padding). Layouts apply `container` once; pages and content inside must NOT re-add `mx-auto max-w-*`.
- **Always render `<Head>`** (from `@/components/seo`, wraps `react-helmet-async`) in a top-level layout so each screen sets its document title.
- **Navigation lives in `DashboardLayout`** via its `navigation` array — add new sidebar links there, using `paths.*.getHref()` and a `lucide-react` icon. Strings use Lingui `` t`...` ``.
- Layouts are **shared components**: no `@/features/*` imports, no data fetching. They take `children`.
- Export every layout from `src/components/layouts/index.ts`.
