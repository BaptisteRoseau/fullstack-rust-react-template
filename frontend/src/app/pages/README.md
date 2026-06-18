# `app/pages/`

Route screens, mapped to paths in `app/router.tsx`. Each page is a **thin** component that wraps a
layout and composes feature components — business logic and data fetching live in `features/*`, not here.

## Layout

```
pages/
├── landing.tsx        # public marketing page (standalone, uses `container` directly)
├── not-found.tsx      # 404
├── auth/              # login.tsx, register.tsx (use AuthLayout)
└── app/               # authenticated screens, rendered inside DashboardLayout via root.tsx
    ├── root.tsx       # the /app outlet parent
    ├── dashboard.tsx
    ├── profile.tsx
    ├── users.tsx
    └── discussions/   # discussions.tsx (list), discussion.tsx (detail) + __tests__/
```

## Conventions

- **`export default`** the page component (the router imports the module's `default`).
- Authenticated pages wrap content in `ContentLayout` (which applies the `container` class — don't re-center).
- Optionally export `clientLoader(queryClient)` / `clientAction(queryClient)` to prefetch/mutate; `router.tsx`'s `convert()` adapts them to React Router's `loader`/`action`.
- Set the document title via the layout's `<Head>`.

See `.claude/skills/frontend-react-page`.
