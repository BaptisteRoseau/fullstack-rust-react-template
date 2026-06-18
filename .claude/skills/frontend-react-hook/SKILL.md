---
name: frontend-react-hook
description: How to create or update a reusable React hook (shared in src/hooks or feature-scoped). Use this when building or editing a frontend custom hook.
---

# Custom Hooks

Two homes, decided by scope:

- **Shared, app-wide** → `src/hooks/use-<name>.ts` with a colocated `__tests__/use-<name>.test.ts`.
- **Feature-specific** → `src/features/<feature>/hooks/use-<name>.ts`.

Naming: `use-kebab-case.ts`, exporting `useКamelCase`.

> A hook that wraps a server request is **not** a hook here — it's an API declaration. Put React
> Query hooks in a feature `api/` file instead (see `frontend-react-api`).

## Pattern (memoize callbacks, return a stable object)

```ts
import * as React from 'react'

export const useDisclosure = (initial = false) => {
    const [isOpen, setIsOpen] = React.useState(initial)

    const open = React.useCallback(() => setIsOpen(true), [])
    const close = React.useCallback(() => setIsOpen(false), [])
    const toggle = React.useCallback(() => setIsOpen((state) => !state), [])

    return { isOpen, open, close, toggle }
}
```

## Rules

- **`React.useCallback`/`useMemo`** for returned functions/values so consumers don't re-render needlessly.
- **Return a named object** (`{ isOpen, open, close }`), not a positional tuple, unless mirroring a React primitive's shape.
- **No JSX** — a hook returns state/handlers, not markup. If you need markup, it's a component.
- Shared hooks stay generic: **no `@/features/*` and no `@/lib/api-client`** imports (that would break the dependency rule and the boundary). Feature hooks may use that feature's `api/`.
- Add a colocated test under `__tests__/` (use `renderHook` from `@/testing/test-utils`).
