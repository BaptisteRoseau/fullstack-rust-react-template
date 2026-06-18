# `hooks/`

**Shared, app-wide** custom hooks. Naming: `use-kebab-case.ts`, exporting `useCamelCase`, with a
colocated `__tests__/`.

- `use-disclosure.ts` — open/close/toggle boolean state (modals, drawers).

## Rules

- Memoize returned callbacks (`React.useCallback`) and return a **named object**, not a positional tuple.
- No JSX — hooks return state/handlers, not markup.
- Shared hooks stay generic: **no `@/features/*` or `@/lib/api-client` imports** (that breaks the dependency rule). A hook that wraps a server request is not a hook here — it's a React Query hook in a feature `api/` file.
- Feature-specific hooks live in `src/features/<name>/hooks/` instead.

See `.claude/skills/frontend-react-hook`.
