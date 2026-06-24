# `testing/mocks/`

MSW mock API. One layer powers **dev** (`browser.ts` worker), **tests** (`server.ts` node server),
and **e2e** (via the root `mock-server.ts`).

| File / folder              | Role                                                                                                   |
| -------------------------- | ------------------------------------------------------------------------------------------------------ |
| `db.ts`                    | `@mswjs/data` in-memory database + `initializeDb` / `persistDb`.                                       |
| `handlers/`                | One file per domain (`auth`, `users`, `discussions`, `comments`, `teams`); `index.ts` aggregates them. |
| `utils.ts`                 | `requireAuth`, `requireAdmin`, `sanitizeUser`, `networkDelay`, `hash`, `AUTH_COOKIE`.                  |
| `server.ts` / `browser.ts` | MSW setup for node / browser.                                                                          |
| `index.ts`                 | `enableMocking()` — gated on `env.ENABLE_API_MOCKING`, called from `main.tsx`.                         |

## Adding a handler

1. Match the URL with `` `${env.API_URL}/...` `` (relative paths won't match).
2. Gate with `requireAuth` / `requireAdmin`, returning `401` on error.
3. `await networkDelay()` first; read/write the mock `db`; `await persistDb('<model>')` after writes.
4. Shape the response like the real API (including the `{ data, meta }` envelope the api-client unwraps).
5. Register the array in `handlers/index.ts`; add the model to `db.ts` and a factory to `../data-generators.ts` if new.

Mocking is enabled with `VITE_APP_ENABLE_API_MOCKING=true`. A test failing with 401/404 usually means
a missing handler — fix it here, don't stub the client.

See `.claude/skills/frontend-react-mocks`.
