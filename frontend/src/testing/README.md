# `testing/`

Test infrastructure for Vitest + Testing Library. One MSW mock layer backs dev, unit/integration
tests, and e2e.

| File / folder | Role |
|---------------|------|
| `test-utils.tsx` | The **only** allowed testing import (ESLint forbids `@testing-library/react` directly). Re-exports Testing Library + `userEvent`, and adds `renderApp` (wraps UI in the real `AppProvider` + a memory router, seeds/logs in a user, waits for loading to clear) plus `createUser`/`createDiscussion`/`loginAsUser`. |
| `data-generators.ts` | Falso factories (`createUser`, `createTeam`, `createDiscussion`, `createComment`), each taking `overrides`. |
| `setup-tests.ts` | Vitest setup (jest-dom matchers, MSW server + DB lifecycle). Auto-loaded via `vite.config.ts`. |
| `mocks/` | MSW server/worker, the `@mswjs/data` DB, request handlers. See its own README. |

## Rules

- **Import from `@/testing/test-utils`**, never `@testing-library/react`.
- Tests are integration-first: render through `renderApp`, drive with `userEvent`, assert visible outcomes. HTTP is **real via MSW** — don't mock `fetch`/`axios`.
- Colocate tests in `__tests__/` beside the code.

See `.claude/skills/frontend-react-testing`.
