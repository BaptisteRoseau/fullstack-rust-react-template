---
name: frontend-component
description: How to create or update a domain-aware shared component under src/components (fetches its own data, reads contexts, references domain types) and how to decide between src/components, src/design-system and a page-local component. Use this when building a cross-page component, layout chrome, or a route guard.
---

# Shared components

`src/components/` holds cross-page components that **are** allowed to know about the domain: they
call API services, read contexts and reference domain types. That is the single distinction from
`src/design-system/`.

## Which layer?

| Question | `design-system/` | `components/` |
|---|---|---|
| Imports from `api/hooks/`? | never | yes |
| Reads a `contexts/` value or a store? | never | yes |
| Mentions a domain type (`CurrentUser`, `ApiKey`)? | never | yes |
| Used by more than one page? | yes | yes — otherwise it belongs to the page |
| Storybook story? | mandatory | when it renders without heavy mocking |

A component used by exactly **one** page lives in that page's `components/` folder. Move it up to
`src/components/` on the second consumer, in the same commit.

## Directory tree

```
src/components/
├── errors/ErrorFallback/       # root error-boundary UI
├── forms/                      # RHF bindings over design-system inputs
│   ├── Form/
│   ├── FormField/
│   └── fields/{TextField,TextAreaField,CheckboxGroupField}/
├── head/Head/                  # document title / meta
├── layout/                     # chrome used by src/layouts/
│   ├── AppHeader/
│   ├── AppFooter/
│   ├── Logo/
│   ├── LocaleSwitcher/
│   └── UserMenu/
├── notifications/{Notifications,Notification}/
├── ConfirmationDialog/
└── ProtectedRoute/
```

Grouping folders are kebab-case; component folders are PascalCase. Inside a component the shape is
identical to the design system's: `Component.tsx`, `Component.test.tsx`, optional
`Component.stories.tsx`, `component.module.scss`, `index.ts`.

Scaffold it rather than hand-writing or copy-pasting a sibling. From `frontend/`:

```bash
bun run generate component components <group> <ComponentName>
# e.g. bun run generate component components layout AppHeader
# no grouping folder:  bun run generate component components "" ProtectedRoute
```

The `components` layer argument is what keeps the story out — this layer only gets one when it
renders without heavy mocking, so add it by hand in that case. Run `bun run generate` with no
arguments to be prompted.

## A domain-aware component

The point of this layer: it reaches for its own data. Handle loading and error before the happy
path — a component that assumes `data` is defined is a crash waiting for a slow network.

```tsx
import { Trans, useLingui } from '@lingui/react/macro'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { Spinner } from '@/design-system/Spinner'

export function AppHeader() {
    const { t } = useLingui()
    const { data: user, isLoading } = useApiCurrentUser()

    if (isLoading) {
        return <Spinner size="sm" label={t`Loading`} />
    }

    return user ? <UserMenu user={user} /> : <SignedOutActions />
}
```

Imports flow `components/` → `design-system/` and `components/` → `api/hooks/`. Never the
reverse.

## Splitting a growing component

Past ~150 lines, split into sibling files inside the same folder rather than creating a parallel
top-level component. The barrel decides what is public. If a sub-part needs its own stylesheet and
test, promote it to a nested folder.

## Route guard

`ProtectedRoute` reads the auth service and redirects, preserving the target:

```tsx
if (!user) {
    return <Navigate to={`${PATHS.login}?redirect=${encodeURIComponent(location.pathname)}`} replace />
}
```

Render it as a pathless wrapper route in `src/router/routes.tsx` so lazy children stay lazy.

## Testing

This is the layer where manual service mocks pay off — mock the service, assert the rendering.

```tsx
import { screen } from '@testing-library/react'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { buildCurrentUser } from '@/test-utils/fixtures/auth'
import { render } from '@/test-utils/render'

import { AppHeader } from './AppHeader'

vi.mock('@/api/hooks/useApiCurrentUser')

it('shows the user name when signed in', () => {
    const user = buildCurrentUser({ firstName: 'Ada', lastName: 'Lovelace' })
    vi.mocked(useCurrentUser).mockReturnValue({
        data: user, error: undefined, isLoading: false, isValidating: false, mutate: vi.fn(),
    })

    render(<AppHeader />)

    expect(
        screen.getByRole('button', { name: /ada lovelace/i }),
        `expected the account trigger for ${user.firstName}, got: ${document.body.textContent}`,
    ).toBeVisible()
})
```

Use `render` from `@/test-utils/render` (not Testing Library's directly) so i18n, SWR and the
router are in place.

## Accessibility

Never put an `aria-label` on an element that already has visible text — it *replaces* the
accessible name and breaks both screen readers and role-based queries. Label landmarks
(`<nav aria-label>`) so multiple navigations stay distinguishable.
