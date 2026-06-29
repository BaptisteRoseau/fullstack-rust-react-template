---
name: frontend-react-testing
description: How to write Vitest + Testing Library integration tests using renderApp and the MSW-backed test-utils. Use this when adding or updating frontend tests, component tests, or feature workflow tests.
---

# Testing (Vitest + Testing Library + MSW)

Integration tests are the primary form: render a feature/page through the real providers, drive it
with `userEvent`, and assert outcomes. **HTTP is real, served by MSW** (the same handlers as dev) —
never mock `fetch`/`axios`. Always import from `@/testing/test-utils`, never `@testing-library/react`
directly (ESLint enforces this).

Tests live in colocated `__tests__/` folders. Setup is `src/testing/setup-tests.ts` (auto-loaded via
`vite.config.ts`), which boots/reset the MSW server and the in-memory DB between tests.

## `renderApp` + data generators

```tsx
import { createDiscussion } from '@/testing/data-generators'
import { renderApp, screen, userEvent, waitFor, within } from '@/testing/test-utils'

test('creates and deletes a discussion', { timeout: 10000 }, async () => {
    // seeds + logs in a user by default; pass `user: null` to render unauthenticated
    await renderApp(<DiscussionsRoute />)

    const newDiscussion = createDiscussion()

    await userEvent.click(screen.getByRole('button', { name: /create discussion/i }))
    const drawer = await screen.findByRole('dialog', { name: /create discussion/i })

    await userEvent.type(within(drawer).getByText(/title/i), newDiscussion.title)
    await userEvent.type(within(drawer).getByText(/body/i), newDiscussion.body)
    await userEvent.click(within(drawer).getByRole('button', { name: /submit/i }))

    await waitFor(() => expect(drawer).not.toBeInTheDocument())
    expect(await screen.findByText(newDiscussion.title)).toBeInTheDocument()
})
```

`renderApp(ui, { user, url, path })`:
- `user` omitted → creates + logs in a fresh user; `user: null` → unauthenticated; `user: someUser` → logs in that user.
- `url` / `path` set up a `createMemoryRouter` so route-dependent components work.
- Wraps `ui` in the real `AppProvider` and waits for loading spinners to clear.

Seed extra data with `createUser` / `createDiscussion` from `@/testing/test-utils` (which write to the mock DB) or build fixtures with the `@/testing/data-generators` factories (`createUser`, `createTeam`, `createDiscussion`, `createComment`), each taking `overrides`.

## Rules

- **Query by role/label/text** (`getByRole`, `findByText`) and assert user-visible outcomes — test behavior, not implementation.
- **`find*` / `waitFor`** for anything async (data load, drawer open/close, toast) — don't assert synchronously after an interaction.
- **Don't mock the API client.** Add/adjust an MSW handler instead (`frontend-react-mocks`); a missing handler means a missing/incorrect mock, not a reason to stub.
- Give long flows an explicit `{ timeout }`.
- Run with `bun run test` (Vitest). E2E (Playwright, `bun run test:e2e`) is separate and runs against `mock-server.ts`.
