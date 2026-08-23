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
│   ├── db.ts               # @msw/data in-memory collections
│   ├── utils.ts            # endpoint(), networkDelay(), session cookie helpers
│   └── handlers/           # one file per domain + index barrel
└── fixtures/               # domain builders (buildApiKey) + wire builders (buildGetApiKeyResponse)
mock-server.ts              # Express + @mswjs/http-middleware, used by e2e
```

The same handlers back three consumers: the browser worker in dev, the node server in unit tests,
and the standalone Express server for e2e. Write a handler once.

## Adding a handler

For a **new domain**, the handler file already exists: `bun run generate api <domainName>
<endpointPath>` writes `src/test-utils/mocks/handlers/<domain>.ts` along with the API layer (see
the `frontend-api` skill). Fill that file in rather than creating one. For a new endpoint on an
existing domain, add the resolver to the domain's handler array.

```ts
import { http, HttpResponse } from 'msw'

import type { GetApiKeyResponse } from '@/api/generated'

import { CURRENT_USER_ID, db, persistDb } from '../db'
import { API_PATHS, endpoint, isAuthenticated, networkDelay } from '../utils'

export const apiKeyHandlers = [
    http.get(endpoint(API_PATHS.apiKeys), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        return HttpResponse.json<GetApiKeyResponse[]>(
            db.apiKey
                .findMany((query) => query.where({ userId: CURRENT_USER_ID }))
                .map(toGetApiKeyResponse),
        )
    }),
]
```

**Handlers emit the wire shape, not the domain shape.** This is the easiest trap in the API layer:
`apiKey.createdAt` is an RFC 3339 string on the wire and a `Date` in the domain, and
`GetMeResponse.createdAt` is an epoch in milliseconds — all three are `Date` once a converter has
run. Type every response with the generated type (`HttpResponse.json<GetApiKeyResponse>(…)`) and let
the compiler hold the line. `src/test-utils/**` is the one place outside `src/api/` allowed to import
`@/api/generated`, for exactly this reason.

Then register it in `handlers/index.ts` — the generator does not touch the barrel.

Rules:

- Build the path with `endpoint(API_PATHS.x)` — `endpoint` prefixes `*` so the handler matches
  whatever origin the caller uses, and `API_PATHS` in `mocks/utils.ts` keeps every URL in one place
  instead of scattering literals.
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

`src/test-utils/mocks/db.ts` declares the collections and seeds the signed-in user
(`CURRENT_USER_ID`, Ada Lovelace). Add a collection there when you add a domain, and extend
`seedDb()` if the domain needs a baseline row.

Collections are `@msw/data` `Collection`s described with a **Zod schema**, so a record's type is the
schema's output type — `permissions` really is `string[]`, and `createdAt` really is the wire's type
for that domain. Queries take a builder rather than a nested object, and the mutating methods
(`create`, `update`, `updateMany`) are async while `findFirst`, `findMany`, `delete` and `deleteMany`
are not:

```ts
db.apiKey.findFirst((query) => query.where({ id: apiKeyId }))
const apiKey = await db.apiKey.create({ name, permissions, userId: CURRENT_USER_ID })
await db.user.update((query) => query.where({ id: CURRENT_USER_ID }), {
    data(draft) {
        Object.assign(draft, body)
    },
})
```

`POST /api/__reset` clears and reseeds the database. The e2e fixture in `e2e/utils/fixtures.ts`
calls it before every test, which is what keeps the suite order-independent — use `test` from that
module, never from `@playwright/test` directly.

## Fixtures

Fixtures are **builders**, not frozen objects, so a test states only what it cares about:

```ts
import { randProductName, randUuid } from '@ngneat/falso'

export function buildApiKey(overrides: Partial<ApiKey> = {}): ApiKey {
    return {
        id: randUuid(),
        name: randProductName(),
        permissions: ['read'],
        createdAt: new Date(),
        ...overrides,
    }
}
```

A domain builds two kinds of builder, and mixing them is the bug this layout exists to catch:

- **domain builders** (`buildApiKey`) for tests that assert on what a component renders;
- **wire builders** (`buildGetApiKeyResponse`), typed with the generated response type, for anything
  handed to `HttpResponse.json`.

`createdAt` is a `Date` in the first and a string in the second. Never feed a domain object to a
handler.

## Which double for which test

| Subject | Double |
|---|---|
| A primitive, hook or util | none |
| A component or page (the UI is the subject) | `vi.mock('@/api/hooks/useApiXxx')` — the automock plus `vi.mocked(...).mockReturnValue(...)` needs no hand-written double |
| A domain fetcher or an api hook (the transport is the subject) | MSW via `server.use(...)` |
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
