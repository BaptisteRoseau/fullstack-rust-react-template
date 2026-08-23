---
name: frontend-mocks
description: Use when adding a mock endpoint, seeding mock data, or fixing a missing or wrong API mock.
---

# Mocks (MSW)

```txt
src/test-utils/
├── server.ts               # MSW node server (unit tests)
├── enableMocking.ts        # MSW browser worker (dev, when VITE_APP_ENABLE_API_MOCKING=true)
├── mocks/
│   ├── db.ts               # @msw/data in-memory collections
│   ├── utils.ts            # endpoint(), API_PATHS, networkDelay(), isAuthenticated()
│   └── handlers/           # one file per domain + index barrel
└── fixtures/                # domain builders (buildApiKey) + wire builders (buildGetApiKeyResponse)
mock-server.ts               # Express + @mswjs/http-middleware, used by e2e
```

The same handlers back three consumers: the browser worker in dev, the node server in unit tests,
and the standalone Express server for e2e. Write a handler once.

## 1. Add or extend the handler file

For a **new domain**, `bun run generate api <domainName> <backendPath>` already writes
`src/test-utils/mocks/handlers/<domain>.ts` along with the API layer — Skill(frontend-api). Fill
that file in rather than creating one. For a new endpoint on an existing domain, add a resolver to
the domain's handler array. See
[src/test-utils/mocks/handlers/apiKeys.ts](../../../frontend/src/test-utils/mocks/handlers/apiKeys.ts)
for the four HTTP-verb shapes: a `GET` list, a `POST` create, a `GET` by id and a `DELETE`.

**Handlers emit the wire shape, not the domain shape.** Type every response with the generated type
(`HttpResponse.json<GetApiKeyResponse>(…)`) and let the compiler hold the line. `src/test-utils/**`
is the one place outside `src/api/` allowed to import `@/api/generated`, for exactly this reason.

Then register the file in `handlers/index.ts` — the generator does not touch the barrel.

Rules:

- Build the path with `endpoint(API_PATHS.x)` — `endpoint` prefixes `*` so the handler matches
  whatever origin the caller uses, and `API_PATHS` in `mocks/utils.ts` keeps every URL in one place.
- Mirror the **real** backend: same status codes, same error body shape (`{ id, error }`), same
  204-with-no-body semantics. Check with the `api-backend` skill.
- Mutating handlers call `persistDb(model)` so dev and e2e survive a reload.

## 2. Check authentication with `isAuthenticated(request)`

Never read the resolver's `cookies` argument — MSW keeps a Node-side cookie jar and merges it into
both `cookies` and the request headers, so a login in one browser context would leak into every
later request. `isAuthenticated` reads the `x-forwarded-cookie` header that `mock-server.ts` copies
from the real Express request before MSW sees it.

## 3. Extend the database, if the domain is new

`src/test-utils/mocks/db.ts` declares the `@msw/data` collections, described with a Zod schema, and
seeds the signed-in user (`CURRENT_USER_ID`, Ada Lovelace). Add a collection when you add a domain,
and extend `seedDb()` if it needs a baseline row. `POST /api/__reset` clears and reseeds the
database — `e2e/utils/fixtures.ts` calls it before every test, which is what keeps the suite
order-independent.

## 4. Write fixtures as builders, not frozen objects

A domain builds two kinds, and mixing them is the bug this layout exists to catch:

- **domain builders** (`buildApiKey`) for tests that assert on what a component renders;
- **wire builders** (`buildGetApiKeyResponse`), typed with the generated response type, for
  anything handed to `HttpResponse.json`.

See [src/test-utils/fixtures/apiKeys.ts](../../../frontend/src/test-utils/fixtures/apiKeys.ts) —
`createdAt` is a `Date` in the first and an RFC 3339 string in the second. Never feed a domain
object to a handler.

## Which double for which test

| Subject | Double |
| --- | --- |
| A primitive, hook or util | none |
| A component or page (the UI is the subject) | `vi.mock('@/api/hooks/useApiXxx')` |
| A domain fetcher or an api hook (the transport is the subject) | MSW via `server.use(...)` |
| A user journey | MSW through `mock-server.ts` (e2e) |

Do not mix both for one subject. See Skill(frontend-testing) for the full test-level table.

## Running the mock backend

```bash
bun run run-mock-server     # Express on VITE_APP_MOCK_API_PORT (8081 for e2e)
```

Point the app at it with `VITE_APP_API_URL=http://localhost:8081` and
`VITE_APP_ENABLE_API_MOCKING=false` — that is what `playwright.config.ts` does. Set
`VITE_APP_ENABLE_API_MOCKING=true` instead to run the worker inside the browser, which intercepts
XHR but **not** full-page navigations, so the OIDC redirect will not be mocked in that mode.

## Checklist

- [ ] The handler is registered in `handlers/index.ts`.
- [ ] Every mock response is typed with the generated (wire) type, not the domain type.
