import { http, HttpResponse } from 'msw'
import { nanoid } from 'nanoid'

import type {
    CreateApiKeyRequest,
    CreateApiKeyResponse,
    GetApiKeyResponse,
} from '@/api/generated'

import type { ApiKeyRecord } from '../db'
import { CURRENT_USER_ID, db, persistDb } from '../db'
import { API_PATHS, endpoint, isAuthenticated, networkDelay } from '../utils'

const UNAUTHORIZED = { id: 'UNAUTHORIZED', error: 'Not authenticated' }

/**
 * The wire shape is built field by field rather than spread so that `userId`,
 * which the record carries but the response does not, cannot leak. `createdAt`
 * is an RFC 3339 string here, not the domain's `Date`.
 */
function toGetApiKeyResponse(apiKey: ApiKeyRecord): GetApiKeyResponse {
    return {
        id: apiKey.id,
        name: apiKey.name,
        permissions: apiKey.permissions,
        createdAt: apiKey.createdAt,
    }
}

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

    http.post(endpoint(API_PATHS.apiKeys), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        const body = (await request.json()) as CreateApiKeyRequest
        const apiKey = await db.apiKey.create({
            name: body.name,
            permissions: body.permissions,
            createdAt: new Date().toISOString(),
            userId: CURRENT_USER_ID,
        })
        await persistDb('apiKey')
        return HttpResponse.json<CreateApiKeyResponse>(
            { ...toGetApiKeyResponse(apiKey), key: `sk_${nanoid(32)}` },
            { status: 201 },
        )
    }),

    http.get(
        endpoint(`${API_PATHS.apiKeys}/:apiKeyId`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const apiKey = db.apiKey.findFirst((query) =>
                query.where({ id: String(params.apiKeyId) }),
            )
            if (!apiKey) {
                return HttpResponse.json(
                    { id: 'NOT_FOUND', error: 'API key not found' },
                    { status: 404 },
                )
            }
            return HttpResponse.json<GetApiKeyResponse>(
                toGetApiKeyResponse(apiKey),
            )
        },
    ),

    http.delete(
        endpoint(`${API_PATHS.apiKeys}/:apiKeyId`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const deleted = db.apiKey.delete((query) =>
                query.where({ id: String(params.apiKeyId) }),
            )
            if (!deleted) {
                return HttpResponse.json(
                    { id: 'NOT_FOUND', error: 'API key not found' },
                    { status: 404 },
                )
            }
            await persistDb('apiKey')
            return HttpResponse.text(null, { status: 204 })
        },
    ),
]
