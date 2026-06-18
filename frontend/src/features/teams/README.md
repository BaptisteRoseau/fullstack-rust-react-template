# `features/teams`

Teams group users; discussions are scoped to a team. A team is created (or joined) during
registration, and the creator becomes its admin.

```
teams/
└── api/   # get-teams — used by the registration form to pick a team to join
```

API-only feature (no components yet). `Team` model is in `@/types/api.ts`.

See `.claude/skills/frontend-react-api`.
