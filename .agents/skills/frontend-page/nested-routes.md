# Nested routes and sub-navigation

Read this only when a page owns child routes, such as a settings screen with tabs.

Use real nested routes, not local tab state. Deep links, the back button and e2e specs then work
without extra code.

## Structure

The parent page renders the shared chrome and an `<Outlet/>`. Each child lives in its own folder
under `sections/`, and the page barrel exports all of them.

[src/pages/User](../../../frontend/src/pages/User) is the reference: `User.tsx` holds the left nav,
`sections/Information` and `sections/ApiKeys` hold the children.

## Route entry

In [src/router/routes.tsx](../../../frontend/src/router/routes.tsx), nest `children` under the
parent entry:

- The default child uses `index: true` and no `path`.
- Every other child gets its own `PATHS` entry.
- Each child keeps its own `lazy` import, pointing at the same page barrel.

To guard a whole branch, wrap it in a **pathless route** —
`{ element: <ProtectedRoute><Outlet /></ProtectedRoute>, children: [...] }`. The children keep their
own `lazy` this way.

`HydrateFallback: () => null` on a lazy root silences React Router's hydration warning.

## Sub-navigation links

Use `NavLink` with a class callback for the active state, and `end` on the index link so it is not
marked active on every child route:

```tsx
<NavLink
    to={PATHS.user.information}
    end
    className={({ isActive }) => clsx(styles.link, isActive && styles.active)}
>
```
