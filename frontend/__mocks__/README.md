# `__mocks__/`

Automatic Vitest module mocks (resolved by filename = package name).

- `zustand.ts` — wraps the real `create` / `createStore` and registers a reset function for **every**
  store, then resets them all in an `afterEach`. This keeps global client state (notifications, etc.)
  isolated between tests so one test's state can't leak into the next. You don't import this — Vitest
  applies it automatically wherever `zustand` is used.
- `vitest-env.d.ts` — ambient type declarations for the test environment.

Add a file here only to mock an entire third-party module across the whole test suite. For API
mocking, use MSW in `src/testing/mocks/` instead (see `.claude/skills/frontend-react-mocks`).
