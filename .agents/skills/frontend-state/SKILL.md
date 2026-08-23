---
name: frontend-state
description: Use when adding global UI state, a toast, a theme toggle, or deciding where a piece of state belongs.
---

# State

Pick the narrowest tool that works. Most "global state" problems are one of the first three rows.

## 1. Choose the tool

| Kind of state | Tool |
| --- | --- |
| Server data | **SWR** — `api/hooks/useApiXxx` hooks. Never copy it into a store — Skill(frontend-api). |
| State used by one component | `useState` |
| State shared between a parent and its children | props |
| State used by one subtree | React context — read [context.md](./context.md) |
| Genuinely app-wide UI state | **Zustand** store in `src/stores/` |

Before adding a store, check it is not one of these mistakes: caching an API response (use SWR),
sharing state between a parent and a child (props), or state used by exactly one subtree (context).

## 2. Generate the store

`src/stores/` currently holds `notifications`, `theme` and `locale` — roughly the whole legitimate
population of this folder.

```bash
bun run generate store <storeName>
```

That writes `src/stores/<storeName>.ts`; fill in the state and actions. See
[src/stores/notifications.ts](../../../frontend/src/stores/notifications.ts) for the reference
shape: a typed state slice plus one action per state transition.

## 3. Select narrowly

```tsx
const addNotification = useNotifications((state) => state.addNotification)
```

A component re-renders only for the slice it selects. Outside React (an interceptor, a helper),
reach the store imperatively: `useNotifications.getState().addNotification(...)`.

## 4. Persist it, if it must survive a reload

`theme` and `locale` mirror into `localStorage` under a key from `src/constants/storage.ts`, and
export a plain reader used by `main.tsx` before React mounts — that avoids a flash of the wrong
theme or language. See [src/stores/locale.ts](../../../frontend/src/stores/locale.ts) for
`storedLocale()`. The theme store writes `document.documentElement.dataset.theme`, which is what
`src/css/_themes.scss` keys its custom properties off.

## Notifications

Raise one from any mutation handler; `<Notifications />` renders them from `src/Context.tsx`. Each
notification renders with `role="alert"` and its title as the accessible name, so tests assert
`getByRole('alert', { name: 'Profile updated' })` — Skill(frontend-testing).

## Checklist

- [ ] The store is not caching something SWR already caches.
- [ ] Call sites select the narrowest slice they need, not the whole store.
