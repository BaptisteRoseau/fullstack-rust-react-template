---
name: frontend-component
description: Use when building a cross-page component, layout chrome, or a route guard under src/components.
---

# Shared components

`src/components/` holds cross-page components that **are** allowed to know about the domain: they
call API services, read contexts and reference domain types. That is the single distinction from
`src/design-system/` — see Skill(frontend-design-system) and Skill(frontend-architecture) for the
full layering rules.

## 1. Decide it belongs here

| Question | `design-system/` | `components/` |
| --- | --- | --- |
| Imports from `api/hooks/`? | never | yes |
| Reads a `contexts/` value or a store? | never | yes |
| Mentions a domain type (`CurrentUser`, `ApiKey`)? | never | yes |
| Used by more than one page? | yes | yes — otherwise it belongs to the page |
| Storybook story? | mandatory | when it renders without heavy mocking |

A component used by exactly **one** page lives in that page's `components/` folder instead. Move it
up to `src/components/` on the second consumer, in the same commit.

## 2. Generate the folder

```bash
bun run generate component components <group-or-empty-string> <ComponentName>
# e.g. bun run generate component components layout AppHeader
# no grouping folder:  bun run generate component components "" ProtectedRoute
```

This writes `Component.tsx`, `Component.test.tsx`, `component.module.scss` and `index.ts`. Do not
hand-write them — see [generators/](../../../frontend/generators/README.md). A story is not
generated for this layer; add one by hand only when the component renders without heavy mocking.
Run `bun run generate` with no arguments to be prompted. See
[src/components](../../../frontend/src/components) for the current groupings (`errors/`, `forms/`,
`head/`, `layout/`, `notifications/`).

## 3. Fetch its own data

The point of this layer: it reaches for its own data. Handle loading and error before the happy
path — a component that assumes `data` is defined is a crash waiting for a slow network. See
[src/components/layout/AppHeader/AppHeader.tsx](../../../frontend/src/components/layout/AppHeader/AppHeader.tsx)
for the reference: it renders a spinner while `useApiCurrentUser` loads, then branches on whether a
user came back.

## 4. Keep it small

Past ~150 lines, split into sibling files inside the same folder rather than creating a parallel
top-level component. The barrel decides what is public. If a sub-part needs its own stylesheet and
test, promote it to a nested folder.

## 5. Test by mocking the hook

This is the layer where `vi.mock('@/api/hooks/useApiXxx')` pays off — mock the hook, assert the
rendering. See
[src/components/layout/AppHeader/AppHeader.test.tsx](../../../frontend/src/components/layout/AppHeader/AppHeader.test.tsx)
and Skill(frontend-testing) for the render helper and assertion style.

## Route guard

`ProtectedRoute` reads the current user and redirects, preserving the target path — see
[src/components/ProtectedRoute/ProtectedRoute.tsx](../../../frontend/src/components/ProtectedRoute/ProtectedRoute.tsx).
Render it as a pathless wrapper route in `src/router/routes.tsx` so lazy children stay lazy —
Skill(frontend-page).

## Accessibility

Never put an `aria-label` on an element that already has visible text — it *replaces* the
accessible name and breaks both screen readers and role-based queries. Label landmarks
(`<nav aria-label>`) so multiple navigations stay distinguishable.

## Checklist

- [ ] A second consumer of a page-local component moved it to `src/components/` in the same commit.
- [ ] Loading and error states are handled before the happy-path render, not assumed away.
