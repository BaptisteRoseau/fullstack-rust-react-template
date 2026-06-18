# `features/users`

User management and profile.

```
users/
├── api/         # get-users, update-profile, delete-user
└── components/  # users-list, update-profile, delete-user
```

- Listing/deleting users is admin-only (`<Authorization allowedRoles={[ROLES.ADMIN]}>`); profile
  editing is available to the current user.
- `update-profile` uses the shared `Form` + Zod pattern; the current user comes from `useUser()`
  (`@/lib/auth`).

`User` model is in `@/types/api.ts`. See `.claude/skills/frontend-react-api`,
`frontend-react-form`, and `frontend-react-authorization`.
