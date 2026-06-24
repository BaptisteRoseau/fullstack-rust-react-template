# `components/layouts/`

Page shells, re-exported from `index.ts`.

| Layout            | Use                                                                                                                             |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `DashboardLayout` | Authenticated shell (sidebar + top bar). Parent of all `/app` routes via `app/pages/app/root.tsx`. Owns the `navigation` array. |
| `ContentLayout`   | Per-page wrapper: sets `<Head>` title, renders the page heading + body inside the `container`. Used by every app page.          |
| `AuthLayout`      | Login / register shell.                                                                                                         |

## Rules

- The **`container` class handles centering** (configured in `tailwind.config.cjs`). Layouts apply it once; pages and inner content must not re-add `mx-auto max-w-*`.
- Always render `<Head>` (from `@/components/seo`) in a top-level layout.
- Add sidebar links to `DashboardLayout`'s `navigation` array, using `paths.*.getHref()`, a `lucide-react` icon, and Lingui `` t`...` ``.
- Layouts are shared components: no `@/features/*`, no data fetching — they take `children`.

See `.claude/skills/frontend-react-layout`.
