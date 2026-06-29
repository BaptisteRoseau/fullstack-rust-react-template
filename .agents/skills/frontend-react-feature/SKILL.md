---
name: frontend-react-feature
description: How to create or extend a self-contained feature module under src/features. Use this when adding a new domain (e.g. "notifications", "projects") or organizing feature-specific api/components/hooks/types.
---

# Creating a Feature Module

A feature is an isolated vertical slice under `src/features/<name>/`. Existing features:
`auth`, `discussions`, `comments`, `teams`, `users`.

## Structure (only create the folders you need)

```
src/features/<name>/
├── api/         # one file per endpoint: schema + fetcher + React Query hook
├── components/  # feature-specific components (compose @/components/ui)
├── hooks/       # feature-specific hooks        (optional)
├── stores/      # feature-specific Zustand store (optional)
├── types/       # feature-specific types         (optional)
└── utils/       # feature-specific utilities      (optional)
```

No empty scaffolding — omit folders the feature doesn't use. No `index.ts` barrel: import concrete files.

## Rules specific to this repo

1. **Never import from another feature.** `src/features/discussions` must not import `src/features/comments`. If shared, lift to `@/components`, `@/lib`, `@/hooks`, or `@/types`. ESLint blocks cross-feature imports.
2. **Register the boundary.** When you add a brand-new feature folder, add it to the `import/no-restricted-paths` zones in `eslint.config.cjs` (mirror the existing per-feature entries) so nothing imports into it.
3. **Domain models go in `@/types/api.ts`**, not the feature, when they are API response shapes shared across features (e.g. `User`, `Discussion`). Feature-local view types stay in the feature's `types/`.
4. **Compose at the page.** Pages in `src/app/pages/**` import feature components and stitch features together — that is the only place features meet.

## Typical build order

1. Add/confirm the domain model in `@/types/api.ts`.
2. Write the API layer (`api/get-*.ts`, `api/create-*.ts`, …) — see `frontend-react-api`.
3. Build components in `components/` using `@/components/ui` primitives — see `frontend-react-component`.
4. Add a mock handler so it runs in dev/tests — see `frontend-react-mocks`.
5. Wire a page + route — see `frontend-react-page`.
6. Add an integration test — see `frontend-react-testing`.

## Example: a feature component composing api + ui

```tsx
import { t, Trans } from '@lingui/macro'
import { Plus } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Form, FormDrawer, Input } from '@/components/ui/form'
import { useNotifications } from '@/components/ui/notifications'
import { Authorization, ROLES } from '@/lib/authorization'

import { createThingInputSchema, useCreateThing } from '../api/create-thing'

export const CreateThing = () => {
    const { addNotification } = useNotifications()
    const createThing = useCreateThing({
        mutationConfig: {
            onSuccess: () => addNotification({ type: 'success', title: t`Thing created` }),
        },
    })
    return (
        <Authorization allowedRoles={[ROLES.ADMIN]}>
            {/* FormDrawer + Form ... see frontend-react-form */}
        </Authorization>
    )
}
```
