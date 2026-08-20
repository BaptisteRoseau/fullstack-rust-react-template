import { http, HttpResponse } from 'msw'
import { nanoid } from 'nanoid'

import { API_KEYS_ENDPOINT, type CreateApiKeyBody } from '@/api/apiKeys'

import { CURRENT_USER_ID, db, persistDb } from '../db'
import { endpoint, isAuthenticated, networkDelay } from '../utils'

const UNAUTHORIZED = { id: 'UNAUTHORIZED', error: 'Not authenticated' }

export const apiKeyHandlers = [
    http.get(endpoint(API_KEYS_ENDPOINT), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        return HttpResponse.json(
            db.apiKey.findMany({
                where: { userId: { equals: CURRENT_USER_ID } },
            }),
        )
    }),

    http.post(endpoint(API_KEYS_ENDPOINT), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        const body = (await request.json()) as CreateApiKeyBody
        const apiKey = db.apiKey.create({
            name: body.name,
            permissions: body.permissions,
            createdAt: new Date().toISOString(),
            userId: CURRENT_USER_ID,
        })
        await persistDb('apiKey')
        return HttpResponse.json(
            { ...apiKey, key: `sk_${nanoid(32)}` },
            { status: 201 },
        )
    }),

    http.delete(
        endpoint(`${API_KEYS_ENDPOINT}/:apiKeyId`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const deleted = db.apiKey.delete({
                where: { id: { equals: String(params.apiKeyId) } },
            })
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
