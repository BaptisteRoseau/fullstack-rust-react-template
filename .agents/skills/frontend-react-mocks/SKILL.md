---
name: frontend-react-mocks
description: How to add or update MSW request handlers, the in-memory mock DB, and data generators that back dev, tests, and e2e. Use this when adding a frontend mock endpoint, mock data, or fixing a missing/incorrect API mock.
---

# API Mocking (MSW + @mswjs/data)

One mock layer powers **dev, Vitest, and Playwright e2e**. A new/changed real endpoint needs a
matching handler here or it will 401/404 everywhere.

```
src/testing/
├── data-generators.ts     # Falso factories: createUser/createTeam/createDiscussion/createComment
└── mocks/
    ├── db.ts              # @mswjs/data in-memory DB + initializeDb/persistDb
    ├── utils.ts           # requireAuth, requireAdmin, sanitizeUser, networkDelay, AUTH_COOKIE, hash
    ├── handlers/          # one file per domain (auth, users, discussions, comments, teams)
    │   └── index.ts       # aggregates all handler arrays
    ├── server.ts          # MSW node server (tests)
    ├── browser.ts         # MSW worker (dev)
    └── index.ts           # enableMocking() — gated on env.ENABLE_API_MOCKING, called from main.tsx
```

Mocking is on when `VITE_APP_ENABLE_API_MOCKING=true` (see `.env`). The standalone `mock-server.ts`
(root) serves the same handlers for Playwright.

## Adding a handler

Match `env.API_URL` exactly, gate auth, hit the mock `db`, and add `networkDelay()`:

```ts
import { HttpResponse, http } from 'msw'

import { env } from '@/config/env'

import { db, persistDb } from '../db'
import { requireAuth, requireAdmin, networkDelay } from '../utils'

export const thingsHandlers = [
    http.get(`${env.API_URL}/things`, async ({ cookies, request }) => {
        await networkDelay()
        const { user, error } = requireAuth(cookies)
        if (error) return HttpResponse.json({ message: error }, { status: 401 })

        const url = new URL(request.url)
        const page = Number(url.searchParams.get('page') || 1)
        const total = db.thing.count({ where: { teamId: { equals: user?.teamId } } })
        const result = db.thing.findMany({
            where: { teamId: { equals: user?.teamId } },
            take: 10,
            skip: 10 * (page - 1),
        })
        return HttpResponse.json({ data: result, meta: { page, total, totalPages: Math.ceil(total / 10) } })
    }),

    http.post(`${env.API_URL}/things`, async ({ request, cookies }) => {
        await networkDelay()
        const { user, error } = requireAdmin(cookies)
        if (error) return HttpResponse.json({ message: error }, { status: 401 })
        const data = (await request.json()) as { title: string; body: string }
        const result = db.thing.create({ ...data, teamId: user?.teamId, authorId: user?.id })
        await persistDb('thing')
        return HttpResponse.json(result)
    }),
]
```

Then:
1. **Add a `thing` model to `db.ts`** (`factory({ thing: { id: primaryKey(...), ... } })`) if it's a new resource.
2. **Register the array in `handlers/index.ts`** (`...thingsHandlers`).
3. **Add a factory in `data-generators.ts`** so tests can seed data.

## Rules

- **URLs use `${env.API_URL}`** — relative paths won't match.
- **Gate every handler** with `requireAuth` / `requireAdmin` from `../utils`, returning `401` on `error`, mirroring the real API's auth.
- **Shape responses like the real API**, including the `{ data, meta }` envelope the api-client unwraps — feature `api/` return types depend on it.
- **`await networkDelay()`** first (realistic async; lets tests assert loading states), and **`await persistDb('<model>')`** after writes.
- **A failing test with a 401/404 usually means a missing handler**, not a reason to mock the client — fix it here.
