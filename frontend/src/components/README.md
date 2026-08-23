# components

Domain-aware components shared by more than one page: they may call API hooks, read contexts and
name domain types. That is the single distinction from `src/design-system/`, which may not. A
component used by exactly one page lives in that page's own `components/` folder instead, and
moves up here only on its second consumer.

```txt
components/
├── <group>/                     # kebab-case grouping folder (errors, forms, head, layout, notifications)
│   └── <ComponentName>/
│       ├── <ComponentName>.tsx
│       ├── <ComponentName>.test.tsx
│       ├── <component-name>.module.scss
│       └── index.ts
├── ConfirmationDialog/           # no grouping folder needed
└── ProtectedRoute/                # route guard, used directly in src/router/routes.tsx
```

Current groupings: `errors/` (root error-boundary UI), `forms/` (React Hook Form bindings over
design-system inputs), `head/` (document title and meta), `layout/` (chrome used by
`src/layouts/`: header, footer, logo, menus, switchers) and `notifications/` (toast rendering).

A component folder never has a `.stories.tsx` unless it renders without heavy mocking — most of
this layer cannot.

## Skills

- [frontend-component](../../../.claude/skills/frontend-component/SKILL.md)
- [frontend-form](../../../.claude/skills/frontend-form/SKILL.md)
