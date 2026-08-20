import { http, HttpResponse } from 'msw'

import {
    ME_ENDPOINT,
    LOGOUT_ENDPOINT,
    REFRESH_ENDPOINT,
    type UpdateProfileBody,
} from '@/api/auth'
import { env } from '@/config/env'

import { CURRENT_USER_ID, db, persistDb } from '../db'
import {
    CLEAR_SESSION_COOKIE,
    endpoint,
    isAuthenticated,
    networkDelay,
    SET_SESSION_COOKIE,
} from '../utils'

const UNAUTHORIZED = { id: 'UNAUTHORIZED', error: 'Not authenticated' }

function currentUser() {
    return db.user.findFirst({ where: { id: { equals: CURRENT_USER_ID } } })
}

function redirectTarget(request: Request) {
    const redirect = new URL(request.url).searchParams.get('redirect')
    return `${env.APP_URL}${redirect ?? '/'}`
}

export const authHandlers = [
    http.get(endpoint('/api/auth/login'), ({ request }) =>
        HttpResponse.text(null, {
            status: 303,
            headers: {
                Location: redirectTarget(request),
                'Set-Cookie': SET_SESSION_COOKIE,
            },
        }),
    ),

    http.get(endpoint('/api/auth/register'), ({ request }) =>
        HttpResponse.text(null, {
            status: 303,
            headers: {
                Location: redirectTarget(request),
                'Set-Cookie': SET_SESSION_COOKIE,
            },
        }),
    ),

    http.post(endpoint(LOGOUT_ENDPOINT), async () => {
        await networkDelay()
        return HttpResponse.text(null, {
            status: 204,
            headers: { 'Set-Cookie': CLEAR_SESSION_COOKIE },
        })
    }),

    http.post(endpoint(REFRESH_ENDPOINT), async ({ request }) => {
        await networkDelay()
        return isAuthenticated(request)
            ? HttpResponse.text(null, { status: 200 })
            : HttpResponse.json(UNAUTHORIZED, { status: 401 })
    }),

    http.get(endpoint(ME_ENDPOINT), async ({ request }) => {
        await networkDelay()
        const user = currentUser()
        if (!isAuthenticated(request) || !user) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        return HttpResponse.json(user)
    }),

    http.patch(endpoint(ME_ENDPOINT), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        const body = (await request.json()) as UpdateProfileBody
        const user = db.user.update({
            where: { id: { equals: CURRENT_USER_ID } },
            data: body,
        })
        await persistDb('user')
        return HttpResponse.json(user)
    }),
]
