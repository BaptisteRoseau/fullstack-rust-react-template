import { http, HttpResponse } from 'msw'

import { isApiError } from '@/api/errors'
import {
    buildCreateApiKeyResponse,
    buildGetApiKeyResponse,
} from '@/test-utils/fixtures/apiKeys'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'

import {
    createApiKey,
    fetchApiKey,
    fetchApiKeys,
    revokeApiKey,
} from './apiKeys'

it('lists the caller keys as domain objects', async () => {
    server.use(
        http.get(endpoint(API_PATHS.apiKeys), () =>
            HttpResponse.json([
                buildGetApiKeyResponse({
                    name: 'CI deploy key',
                    createdAt: '2026-01-15T09:30:00Z',
                }),
            ]),
        ),
    )

    const apiKeys = await fetchApiKeys()

    expect(apiKeys.length, `expected 1 key, got ${apiKeys.length}`).toBe(1)
    expect(
        apiKeys[0].createdAt instanceof Date,
        'the fetcher must hand back the converted shape, not the wire one',
    ).toBe(true)
})

it('fetches one key by id', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.apiKeys}/:id`), ({ params }) =>
            HttpResponse.json(
                buildGetApiKeyResponse({ id: String(params.id) }),
            ),
        ),
    )

    const apiKey = await fetchApiKey('key-42')

    expect(apiKey.id, `expected key-42, got ${apiKey.id}`).toBe('key-42')
})

it('creates a key and exposes its secret once', async () => {
    server.use(
        http.post(endpoint(API_PATHS.apiKeys), () =>
            HttpResponse.json(
                buildCreateApiKeyResponse({ key: 'sk_live_1234' }),
                { status: 201 },
            ),
        ),
    )

    const created = await createApiKey({
        name: 'CI deploy key',
        permissions: ['read'],
    })

    expect(created.secret, `expected the raw key, got ${created.secret}`).toBe(
        'sk_live_1234',
    )
})

it('revokes a key without a payload', async () => {
    let revokedId: string | undefined
    server.use(
        http.delete(endpoint(`${API_PATHS.apiKeys}/:id`), ({ params }) => {
            revokedId = String(params.id)
            return HttpResponse.text(null, { status: 204 })
        }),
    )

    await revokeApiKey('key-42')

    expect(revokedId, `expected key-42 to be deleted, got ${revokedId}`).toBe(
        'key-42',
    )
})

it('raises a typed error when a key is missing', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.apiKeys}/:id`), () =>
            HttpResponse.json(
                { error: 'Not found.', id: 'NOT_FOUND' },
                { status: 404 },
            ),
        ),
    )

    const error = await fetchApiKey('missing').catch((thrown) => thrown)

    expect(isApiError(error), `expected an ApiError, got ${error}`).toBe(true)
    expect(error.id, `expected NOT_FOUND, got ${error.id}`).toBe('NOT_FOUND')
})
