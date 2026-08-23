# Scoped React context

Read this only when state belongs to one subtree — a multi-step wizard, a page-level filter — not
for app-wide UI state (use a Zustand store instead).

Contexts have no generator; write these four files by hand under `src/contexts/<name>/`:

```txt
src/contexts/<name>/
├── <Name>Context.ts            # createContext<Value>('<Name>')
├── <Name>ContextProvider.tsx
├── types.ts
└── index.ts
```

Use the typed factory in `src/utils/createContext.tsx` — it removes the `| undefined` dance and
throws a real error when the provider is missing:

```ts
export const [AuthProvider, useAuth] = createContext<AuthContextValue>('Auth')
```

A context that caches an API response is duplicating the cache SWR already gives you — use
Skill(frontend-api) instead.
