---
name: frontend-page
description: Use when adding, moving or removing a page, route or screen in the React frontend.
---

# Add a frontend page

A page is one folder under `src/pages/` plus one entry in the router. Run every command from
`frontend/`.

## 1. Declare the path

Add the route to `PATHS` in
[src/router/constants.ts](../../../frontend/src/router/constants.ts). This is the only file where a
URL literal may appear.
Never write a URL literal in a component.

```ts
export const PATHS = {
    home: '/',
    login: '/auth/login',
    register: '/auth/register',
    user: {
        root: '/user',
        information: '/user',
        apiKeys: '/user/api-keys',
    },
    notFound: '*',
} as const
```

## 2. Generate the folder

```bash
bun run generate page <PageName> "<Page title>"
```

It writes `<PageName>.tsx`, `<page-name>.module.scss`, `<PageName>.test.tsx` and `index.ts`. Do not
hand-write them. See [generators/](../../../frontend/generators/README.md).
Only update them after generation.

## 3. Register the route

Add a lazy entry in [src/router/routes.tsx](../../../frontend/src/router/routes.tsx), copying the
shape of the entries already there: `path` taken from `PATHS`, `lazy` importing the page barrel.

Choose the layout:

- `AppLayout` — header, page, footer. The default.
- `AuthLayout` — centred card, for `/auth/*` only.

Put the entry inside the `ProtectedRoute` wrapper when the page requires a signed-in user.

For child routes and sub-navigation, read [nested-routes.md](./nested-routes.md).

## 4. Write the page component

A page **composes**, it does not implement.

- Data comes from `api/hooks/` — Skill(frontend-api).
- UI comes from `src/components/` and `src/design-system/` — Skill(frontend-component).
- Wrap the body in `ContentLayout` for the title, description and action bar.
- Set the document title with [Head](../../../frontend/src/components/head/Head), passing a
  translated `title`.
- Pass `isLoading` and `error` **down** to the component that renders them. Do not chain early
  returns in the page.

[src/pages/User/sections/ApiKeys](../../../frontend/src/pages/User/sections/ApiKeys) is the
reference to copy.

Give every top-level section the `container` mixin. It holds the section between the page margins
while a full-bleed background still spans the screen:

```scss
.hero {
    padding-block: $space-16;

    @include container;
}
```

Never nest one container inside another. Anything under `ContentLayout` is already contained.

## 5. Keep page-local code private

Only `index.ts` may be imported from outside the page folder.

```txt
pages/User/
├── User.tsx
├── index.ts        # the only public surface
├── components/     # used by this page only
└── sections/       # one folder per child route
```

- `components/` and `sections/` entries have no generator of their own. Mirror what
  `bun run generate component` produces.
- When a second page needs one of them, move it to `src/components/` in the same commit.
- **No page imports another page.** ESLint enforces this.

## 6. Make it reachable

- Add a nav link if a user should be able to find the page. Use `PATHS`, never a literal.
- Wrap every user-facing string in a Lingui macro — Skill(frontend-i18n).
- Fill in the generated test — Skill(frontend-testing). Add an `e2e/` spec if the page is part of a
  critical journey.
- Regenerate the SEO files so the route reaches the sitemap — Skill(frontend-seo).

## Checklist

```bash
.claude/skills/frontend-page/scripts/check_page.sh <PageName>
cd frontend && bun run i18n:check && bun run seo:check
```

- [ ] The page sits under the right layout, and behind `ProtectedRoute` if it needs a session.
- [ ] Nothing outside the page folder imports its internals.
