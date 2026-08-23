import { http, HttpResponse } from 'msw'

import type { GetMeResponse, PatchMeRequest } from '@/api/generated'
import { env } from '@/config/env'

import { CURRENT_USER_ID, db, persistDb } from '../db'
import {
    API_PATHS,
    CLEAR_SESSION_COOKIE,
    endpoint,
    isAuthenticated,
    networkDelay,
    SET_SESSION_COOKIE,
} from '../utils'

const UNAUTHORIZED = { id: 'UNAUTHORIZED', error: 'Not authenticated' }

function currentUser() {
    return db.user.findFirst((query) => query.where({ id: CURRENT_USER_ID }))
}

function redirectTarget(request: Request) {
    const redirect = new URL(request.url).searchParams.get('redirect')
    return `${env.APP_URL}${redirect ?? '/'}`
}

export const authHandlers = [
    http.get(endpoint(API_PATHS.login), ({ request }) =>
        HttpResponse.text(null, {
            status: 303,
            headers: {
                Location: redirectTarget(request),
                'Set-Cookie': SET_SESSION_COOKIE,
            },
        }),
    ),

    http.get(endpoint(API_PATHS.register), ({ request }) =>
        HttpResponse.text(null, {
            status: 303,
            headers: {
                Location: redirectTarget(request),
                'Set-Cookie': SET_SESSION_COOKIE,
            },
        }),
    ),

    http.post(endpoint(API_PATHS.logout), async () => {
        await networkDelay()
        return HttpResponse.text(null, {
            status: 204,
            headers: { 'Set-Cookie': CLEAR_SESSION_COOKIE },
        })
    }),

    http.post(endpoint(API_PATHS.refresh), async ({ request }) => {
        await networkDelay()
        return isAuthenticated(request)
            ? HttpResponse.text(null, { status: 200 })
            : HttpResponse.json(UNAUTHORIZED, { status: 401 })
    }),

    http.get(endpoint(API_PATHS.me), async ({ request }) => {
        await networkDelay()
        const user = currentUser()
        if (!isAuthenticated(request) || !user) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        return HttpResponse.json<GetMeResponse>(user)
    }),

    http.patch(endpoint(API_PATHS.me), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        const body = (await request.json()) as PatchMeRequest
        const user = await db.user.update(
            (query) => query.where({ id: CURRENT_USER_ID }),
            {
                data(draft) {
                    Object.assign(draft, body)
                },
            },
        )
        if (!user) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        await persistDb('user')
        return HttpResponse.json<GetMeResponse>(user)
    }),
]
