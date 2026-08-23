# src

Layer-first source tree. Dependencies flow downwards only: a folder may import from a folder below
it in the list, never from one above.

```txt
src/
├── api/            # generated SDK, domain converters, SWR hooks
├── components/     # domain-aware shared components
├── config/         # env.ts, validated environment variables
├── constants/      # app-wide constants that are not routes
├── css/            # global SCSS: tokens, themes, mixins, reset
├── design-system/  # domain-agnostic UI primitives
├── hooks/          # reusable hooks with no domain knowledge
├── i18n/           # Lingui instance and PO catalogs
├── img/            # static images and SVGs, imported directly by components
├── layouts/        # page shells rendered by the router (AppLayout, AuthLayout, ContentLayout)
├── pages/          # one folder per route
├── router/         # route objects and PATHS
├── stores/         # Zustand stores for app-wide UI state
├── test-utils/     # render helpers, MSW mocks, fixtures
├── types/          # cross-layer TypeScript types
├── utils/          # pure helpers: no JSX, no hooks
├── App.tsx         # mounts the router
├── Context.tsx     # the single provider tree (error boundary, i18n, SWR, notifications)
└── main.tsx        # runtime bootstrap
```

| Folder | May import |
| --- | --- |
| `api/` | nothing from the UI layers |
| `design-system/` | `utils/`, `types/`, `css/`, `hooks/`, Radix |
| `components/` | `design-system/`, `api/domains/<domain>/`, `api/hooks/`, `hooks/`, `stores/` |
| `layouts/` | `components/`, `design-system/` |
| `pages/` | everything, but never another page |
| `router/` | `pages/`, `layouts/`, `components/` |
| `hooks/` | `utils/`, `types/` |
| `utils/` | `types/`, `constants/` |
| `stores/` | `constants/`, `i18n/` |
| `test-utils/` | everything |

Scoped React context state, when a subtree needs it, lives in `contexts/<name>/` — created on
first use, so the folder does not exist yet.

Each folder listed above with its own README describes it in more depth.

## Skills

- [frontend-architecture](../../.claude/skills/frontend-architecture/SKILL.md)
