---
name: frontend-new-page
description: Step-by-step guide for adding a new page to the frontend. Use when asked to create a new route, page, or view.
---

# Adding a New Frontend Page

## 1. Add the path to `src/config/paths.ts`

Add an entry under the appropriate namespace (`app`, `auth`, or top-level):

```ts
app: {
    // ...existing paths...
    myFeature: {
        path: 'my-feature',
        getHref: () => '/app/my-feature',
    },
}
```

For dynamic segments:
```ts
myFeature: {
    path: 'my-feature/:id',
    getHref: (id: string) => `/app/my-feature/${id}`,
},
```

## 2. Create the page file

Place the file at `src/app/pages/app/my-feature.tsx` (or `src/app/pages/my-feature.tsx` for top-level routes).

### Standard page pattern (authenticated app page)

```tsx
import { ContentLayout } from '@/components/layouts'

const MyFeatureRoute = () => {
    return (
        <ContentLayout title="My Feature">
            {/* page content */}
        </ContentLayout>
    )
}

export default MyFeatureRoute
```

`ContentLayout` already applies the `container` class (centered, responsive max-width, `padding: 2rem` horizontally — configured in `tailwind.config.cjs`). Do not add a manual `mx-auto max-w-7xl` wrapper inside it.

### Page with a data loader

```tsx
import { QueryClient } from '@tanstack/react-query'
import { ContentLayout } from '@/components/layouts'
import { getMyDataQueryOptions } from '@/features/my-feature/api/get-my-data'

export const clientLoader = (queryClient: QueryClient) => async () => {
    const query = getMyDataQueryOptions()
    return (
        queryClient.getQueryData(query.queryKey) ??
        (await queryClient.fetchQuery(query))
    )
}

const MyFeatureRoute = () => {
    return (
        <ContentLayout title="My Feature">
            {/* page content */}
        </ContentLayout>
    )
}

export default MyFeatureRoute
```

### Standalone page (no layout — e.g. landing, 404)

Wrap content in a `container` div directly:

```tsx
const MyStandalonePage = () => {
    return (
        <div className="container py-12">
            {/* page content */}
        </div>
    )
}

export default MyStandalonePage
```

## 3. Register the route in `src/app/router.tsx`

### Authenticated app route (under `/app`)

Add a child route inside the `paths.app.root.path` children array:

```ts
{
    path: paths.app.myFeature.path,
    lazy: () =>
        import('./pages/app/my-feature').then(convert(queryClient)),
},
```

### Top-level route

Add a new entry at the top level of the router array:

```ts
{
    path: paths.myFeature.path,
    lazy: () => import('./pages/my-feature').then(convert(queryClient)),
},
```

## 4. Add navigation link (if needed)

If the page should appear in the sidebar, add it to the `navigation` array in `src/components/layouts/dashboard-layout.tsx`:

```ts
import { MyIcon } from 'lucide-react'

const navigation = [
    // ...existing items...
    {
        name: 'My Feature',
        to: paths.app.myFeature.getHref(),
        icon: MyIcon,
    },
]
```

## Container class

The `container` class is configured in `tailwind.config.cjs` with:
- `center: true` → `mx-auto`
- `padding: '2rem'` → `px-8` (horizontal)
- Breakpoint max-widths: `sm: 640px`, `md: 768px`, `lg: 1024px`, `xl: 1280px`, `2xl: 1536px`

`ContentLayout` already wraps its content in `container` divs — no manual centering needed inside it.
