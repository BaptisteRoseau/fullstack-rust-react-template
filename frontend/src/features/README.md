# `features/`

Self-contained **domain slices**. Each feature owns its API calls, components, and (optionally)
hooks/stores/types/utils.

Current features: `auth`, `comments`, `discussions`, `teams`, `users`.

## Per-feature structure (only the folders it needs)

```
features/<name>/
├── api/         # one file per endpoint: Zod schema + Axios fetcher + React Query hook
├── components/  # feature-specific components (compose @/components/ui)
├── hooks/       # feature-specific hooks        (optional)
├── stores/      # feature-specific Zustand store (optional)
├── types/       # feature-specific types         (optional)
└── utils/       # feature-specific utilities      (optional)
```

## The golden rule

**Features never import from each other.** `discussions` must not import `comments`. Shared code is
lifted into `@/components`, `@/lib`, `@/hooks`, or `@/types`; features are composed only at the `app`
layer. This is enforced by ESLint `import/no-restricted-paths` — when you add a new feature folder,
add a matching zone in `eslint.config.cjs`.

No `index.ts` barrels — import concrete files (`../api/create-discussion`).

API response models shared across features live in `@/types/api.ts`, not inside a feature.

See `.claude/skills/frontend-react-feature` and `.claude/skills/frontend-react-api`.
