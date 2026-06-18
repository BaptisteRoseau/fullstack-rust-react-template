# `features/discussions`

CRUD for discussions (a team's topics). The most complete feature — use it as the reference for the
api + components pattern.

```
discussions/
├── api/         # get-discussions (list), get-discussion (one), create/update/delete-discussion
└── components/  # discussions-list, discussion-view, create/update/delete-discussion
```

- **api/** — each file: Zod schema (mutations) + Axios fetcher + React Query hook, with a
  `queryOptions` factory. Mutations invalidate `['discussions']` on success.
- **components/** — compose `@/components/ui` (`Table`, `Form`, `FormDrawer`, `ConfirmationDialog`,
  `Button`) and gate admin actions with `<Authorization allowedRoles={[ROLES.ADMIN]}>`.

Domain model `Discussion` is in `@/types/api.ts`. See `.claude/skills/frontend-react-api`,
`frontend-react-form`, and `frontend-react-feature`.
