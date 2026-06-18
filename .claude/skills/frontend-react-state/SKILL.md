---
name: frontend-react-state
description: How to create or update global client-side state with Zustand (modals, notifications, themes), and when NOT to. Use this when adding a frontend store, global UI state, or notification/toast behavior.
---

# Client State (Zustand)

**Server state is React Query (in feature `api/` files) — not Zustand.** Use Zustand only for global
*client* state: notifications, modals, theme. Default to local `useState`; lift to a store only when
state is genuinely shared across unrelated components.

The reference store is `src/components/ui/notifications/notifications-store.ts`.

## Pattern

```ts
import { nanoid } from 'nanoid'
import { create } from 'zustand'

export type Notification = {
    id: string
    type: 'info' | 'warning' | 'success' | 'error'
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
        set((state) => ({ notifications: [...state.notifications, { id: nanoid(), ...notification }] })),
    dismissNotification: (id) =>
        set((state) => ({ notifications: state.notifications.filter((n) => n.id !== id) })),
}))
```

## Usage

```tsx
// In a component (subscribes to re-renders):
const { addNotification } = useNotifications()
addNotification({ type: 'success', title: t`Saved` })

// Outside React (e.g. the Axios interceptor in lib/api-client.ts):
useNotifications.getState().addNotification({ type: 'error', title: 'Error', message })
```

## Rules

- **Type the full store** (state + actions) and pass it to `create<Store>()`.
- **Immutable updates** via `set((state) => ...)` — spread, never mutate.
- **Read outside React** with `useStore.getState()` (this is how non-component code like the api-client emits toasts).
- **To show a toast, reuse `useNotifications`** — don't build a parallel notification system.
- **Don't put server data in Zustand.** Cache server data with React Query and derive UI state locally.
- Co-locate the store with the feature/component that owns it; promote to a shared location only if multiple features use it.
