# `features/comments`

Comments within a discussion.

```
comments/
├── api/         # get-comments (paginated), create-comment, delete-comment
└── components/  # comments (container), comments-list, create-comment, delete-comment
```

- Comments are scoped to a `discussionId` — fetchers and query keys include it.
- Delete is gated by **PBAC**: `POLICIES['comment:delete'](user, comment)` (admins, or the comment's
  own author) via `<Authorization policyCheck={...}>` — see `.claude/skills/frontend-react-authorization`.

Domain model `Comment` lives in `@/types/api.ts`. See `.claude/skills/frontend-react-api`.
