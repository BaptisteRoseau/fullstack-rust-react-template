---
name: frontend-state
description: How to choose between SWR, React context, Zustand and useState, and how to add a store under src/stores (notifications, theme, locale). Use this when adding global UI state, a toast, a theme toggle, or when deciding where a piece of state belongs.
---

# State

Pick the narrowest tool that works. Most "global state" problems are one of the first three rows.

| Kind of state | Tool |
|---|---|
| Server data | **SWR** — `api/hooks/useApiXxx` hooks. Never copy it into a store. |
| State used by one component | `useState` |
| State shared between a parent and its children | props |
| State used by one subtree | React context in `src/contexts/<name>/` |
| Genuinely app-wide UI state | **Zustand** store in `src/stores/` |

Before adding a store, check it is not one of these mistakes: caching an API response (use SWR),
sharing state between a parent and a child (props), or state used by exactly one subtree (context).

## Zustand stores

`src/stores/` currently holds `notifications`, `theme` and `locale` — that is roughly the whole
legitimate population of this folder.

Once the checks above say a store is really warranted, generate the file from `frontend/`:

```bash
bun run generate store <storeName>
# e.g. bun run generate store notifications
```

That writes `src/stores/<storeName>.ts`; fill in the state and actions.

```ts
import { nanoid } from 'nanoid'
import { create } from 'zustand'

export type Notification = {
    id: string
    type: 'info' | 'success' | 'warning' | 'error'
    title: string
    message?: string
}

type NotificationsStore = {
    notifications: Notification[]
    addNotification: (notification: Omit<Notification, 'id'>) => void
    dismissNotification: (id: string) => void
}

export const useNotifications = create<NotificationsStore>((set) => ({
    notifications: [],
    addNotification: (notification) =>
        set((state) => ({
            notifications: [...state.notifications, { id: nanoid(), ...notification }],
        })),
    dismissNotification: (id) =>
        set((state) => ({
            notifications: state.notifications.filter((n) => n.id !== id),
        })),
}))
```

Select narrowly at the call site so a component re-renders only for what it uses:

```tsx
const addNotification = useNotifications((state) => state.addNotification)
```

Outside React (an interceptor, a helper) reach the store imperatively:

```ts
useNotifications.getState().addNotification({ type: 'error', title: 'Error' })
```

## Notifications

Raise one from any mutation handler; `<Notifications />` renders them from `src/Context.tsx`.

```tsx
addNotification({
    type: 'error',
    title: t`Could not revoke the API key`,
    message: apiErrorMessage(error),
})
```

Each notification renders with `role="alert"` and its title as the accessible name, so tests assert
`getByRole('alert', { name: 'Profile updated' })`.

## Persisted stores

`theme` and `locale` mirror into `localStorage` under a key from `src/constants/storage.ts`, and
export a plain reader used by `main.tsx` before React mounts — that avoids a flash of the wrong
theme or language:

```ts
export function storedLocale(): Locale {
    const stored = window.localStorage.getItem(LOCALE_STORAGE_KEY)
    return locales.includes(stored as Locale) ? (stored as Locale) : defaultLocale
}
```

The theme store writes `document.documentElement.dataset.theme`, which is what
`src/css/_themes.scss` keys its custom properties off.

## Scoped context

For state that belongs to one subtree (a multi-step wizard, a page-level filter), use a context
folder rather than a store:

```
src/contexts/<name>/
├── <Name>Context.ts            # createContext<Value>('<Name>')
├── <Name>ContextProvider.tsx
├── types.ts
└── index.ts
```

Contexts have no generator — write these four files by hand.

Use the typed factory in `@/utils/createContext` — it removes the `| undefined` dance and throws a
real error when the provider is missing:

```ts
export const [AuthProvider, useAuth] = createContext<AuthContextValue>('Auth')
```

A context that caches an API response is duplicating the cache SWR already gives you.
