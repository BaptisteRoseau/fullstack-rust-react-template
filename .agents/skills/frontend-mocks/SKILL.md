---
name: frontend-mocks
description: How to add or update MSW request handlers, the in-memory mock DB and fixtures that back dev, unit tests and e2e. Use this when adding a mock endpoint, seeding mock data, or fixing a missing or wrong API mock.
---

# Mocks (MSW)

```
src/test-utils/
├── server.ts               # MSW node server (unit tests)
├── enableMocking.ts        # MSW browser worker (dev, when VITE_APP_ENABLE_API_MOCKING=true)
├── mocks/
│   ├── browser.ts
│   ├── db.ts               # @mswjs/data in-memory database
│   ├── utils.ts            # endpoint(), networkDelay(), session cookie helpers
│   └── handlers/           # one file per domain + index barrel
└── fixtures/               # buildCurrentUser(), buildApiKey()
mock-server.ts              # Express + @mswjs/http-middleware, used by e2e
```

The same handlers back three consumers: the browser worker in dev, the node server in unit tests,
and the standalone Express server for e2e. Write a handler once.

## Adding a handler

```ts
import { http, HttpResponse } from 'msw'

import { API_KEYS_ENDPOINT, type CreateApiKeyBody } from '@/api/apiKeys'

import { CURRENT_USER_ID, db, persistDb } from '../db'
import { endpoint, isAuthenticated, networkDelay } from '../utils'

export const apiKeyHandlers = [
    http.get(endpoint(API_KEYS_ENDPOINT), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        return HttpResponse.json(
            db.apiKey.findMany({ where: { userId: { equals: CURRENT_USER_ID } } }),
        )
    }),
]
```

Then register it in `handlers/index.ts`.

Rules:

- Build the path with `endpoint(PATH_CONSTANT)` — it prefixes `*` so the handler matches whatever
  origin the caller uses, and it derives the URL from `src/api/<domain>.ts` rather than a literal.
- Mirror the **real** backend: same status codes, same error body shape
  (`{ id, error }`), same 204-with-no-body semantics. Check with the `api-backend` skill.
- Mutating handlers call `persistDb(model)` so dev and e2e survive a reload.

## Authentication in mocks

The mock session is a `mock_session` cookie. `GET /api/auth/{login,register}` answer `303` with
`Set-Cookie` and a `Location` back into the app, which is exactly the shape of the real
backend-for-frontend redirect — so the frontend code path is identical.

**Always check auth with `isAuthenticated(request)`, never with the resolver's `cookies`
argument.** MSW keeps a Node-side cookie jar and merges it into both `cookies` and the request
headers, so a login in one browser context would leak into every later request. `isAuthenticated`
reads the `x-forwarded-cookie` header that `mock-server.ts` copies from the real Express request
before MSW sees it.

## The database

`src/test-utils/mocks/db.ts` declares the models and seeds the signed-in user (`CURRENT_USER_ID`,
Ada Lovelace). Add a model there when you add a domain, and extend `seedDb()` if the domain needs
a baseline row.

`POST /api/__reset` clears and reseeds the database. The e2e fixture in `e2e/utils/fixtures.ts`
calls it before every test, which is what keeps the suite order-independent — use `test` from that
module, never from `@playwright/test` directly.

## Fixtures

Fixtures are **builders**, not frozen objects, so a test states only what it cares about:

```ts
import { randEmail, randUuid } from '@ngneat/falso'

export function buildApiKey(overrides: Partial<ApiKey> = {}): ApiKey {
    return {
        id: randUuid(),
        name: randProductName(),
        permissions: ['read'],
        createdAt: new Date().toISOString(),
        ...overrides,
    }
}
```

## Which double for which test

| Subject | Double |
|---|---|
| A primitive, hook or util | none |
| A component or page (the UI is the subject) | manual `__mocks__` + `vi.mock` |
| A service hook (the transport is the subject) | MSW via `server.use(...)` |
| A user journey | MSW through `mock-server.ts` (e2e) |

Do not mix both for one subject.

## Running the mock backend

```bash
bun run run-mock-server     # Express on VITE_APP_MOCK_API_PORT (8081 for e2e)
```

Point the app at it with `VITE_APP_API_URL=http://localhost:8081` and
`VITE_APP_ENABLE_API_MOCKING=false` — that is what `playwright.config.ts` does. Set
`VITE_APP_ENABLE_API_MOCKING=true` instead to run the worker inside the browser, which intercepts
XHR but **not** full-page navigations, so the OIDC redirect will not be mocked in that mode.
