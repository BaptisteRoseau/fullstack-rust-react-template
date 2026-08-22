import { http, HttpResponse } from 'msw'

import { listApiKeys, me } from '@/api/generated'
import { endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'

import { apiCall } from './client'
import { isApiError } from './errors'

it('unwraps a successful call to its payload', async () => {
    server.use(
        http.get(endpoint('/api/api-key'), () =>
            HttpResponse.json([
                {
                    createdAt: '2026-01-15T00:00:00Z',
                    id: 'key-1',
                    name: 'CI deploy key',
                    permissions: ['read'],
                },
            ]),
        ),
    )

    const apiKeys = await apiCall(() => listApiKeys())

    expect(apiKeys.length, `expected 1 key, got ${apiKeys.length}`).toBe(1)
    expect(
        apiKeys[0].name,
        `expected "CI deploy key", got "${apiKeys[0].name}"`,
    ).toBe('CI deploy key')
})

it('throws a typed ApiError carrying the backend cause', async () => {
    server.use(
        http.get(endpoint('/api/auth/me'), () =>
            HttpResponse.json(
                { error: 'Not authenticated', id: 'UNAUTHORIZED' },
                { status: 401 },
            ),
        ),
        http.post(endpoint('/api/auth/refresh'), () =>
            HttpResponse.json(
                { error: 'Not authenticated', id: 'UNAUTHORIZED' },
                { status: 401 },
            ),
        ),
    )

    const error = await apiCall(() => me()).catch((thrown) => thrown)

    expect(isApiError(error), `expected an ApiError, got ${error}`).toBe(true)
    expect(error.id, `expected UNAUTHORIZED, got ${error.id}`).toBe(
        'UNAUTHORIZED',
    )
    expect(error.status, `expected 401, got ${error.status}`).toBe(401)
})

it('normalises an unreachable server to a NETWORK error', async () => {
    server.use(http.get(endpoint('/api/api-key'), () => HttpResponse.error()))

    const error = await apiCall(() => listApiKeys()).catch((thrown) => thrown)

    expect(isApiError(error), `expected an ApiError, got ${error}`).toBe(true)
    expect(error.id, `expected NETWORK, got ${error.id}`).toBe('NETWORK')
})

it('refreshes the session once and replays the request on a 401', async () => {
    let refreshes = 0
    let profileRequests = 0

    server.use(
        http.get(endpoint('/api/auth/me'), () => {
            profileRequests += 1
            return refreshes === 0
                ? HttpResponse.json(
                      { error: 'Not authenticated', id: 'TOKEN_EXPIRED' },
                      { status: 401 },
                  )
                : HttpResponse.json({
                      createdAt: 0,
                      email: 'ada@example.com',
                      firstName: 'Ada',
                      id: 'user-1',
                      lastName: 'Lovelace',
                      role: 'USER',
                      teamId: '',
                  })
        }),
        http.post(endpoint('/api/auth/refresh'), () => {
            refreshes += 1
            return HttpResponse.text(null, { status: 200 })
        }),
    )

    const profile = await apiCall(() => me())

    expect(refreshes, `expected exactly 1 refresh, got ${refreshes}`).toBe(1)
    expect(
        profileRequests,
        `expected the request to be replayed once, got ${profileRequests} attempts`,
    ).toBe(2)
    expect(
        profile.email,
        `expected the replayed response, got ${profile.email}`,
    ).toBe('ada@example.com')
})
