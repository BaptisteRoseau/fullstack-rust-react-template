# pages

One folder per route. A page folder is private: nothing outside it may import from its
`components/`, `hooks/` or `sections/`, and no page imports another page — ESLint enforces both.
Only `index.ts` is the public surface, used by `src/router/routes.tsx`.

```txt
pages/
└── <PageName>/
    ├── <PageName>.tsx
    ├── <PageName>.test.tsx
    ├── <page-name>.module.scss
    ├── index.ts                # the only public surface
    ├── components/             # used by this page only, no generator of its own
    └── sections/                # one folder per child route, for a page with sub-navigation
        └── <SectionName>/
```

Current pages: `Home`, `Login`, `Register`, `NotFound`, and `User` (which owns `sections/` for its
tabs and `components/` for the API-key management UI beneath it).

A component moves out of a page's `components/` folder into `src/components/` the moment a second
page needs it.

## Skills

- [frontend-page](../../../.claude/skills/frontend-page/SKILL.md)
- [frontend-architecture](../../../.claude/skills/frontend-architecture/SKILL.md)
