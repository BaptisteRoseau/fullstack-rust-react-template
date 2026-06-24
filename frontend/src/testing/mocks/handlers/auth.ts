import Cookies from 'js-cookie'
import { HttpResponse, http } from 'msw'

import { env } from '@/config/env'

import { requireAuth, AUTH_COOKIE, networkDelay } from '../utils'

export const authHandlers = [
    // GET /auth/me — return the currently authenticated user
    http.get(`${env.API_URL}/auth/me`, async ({ cookies }) => {
        await networkDelay()

        const { user, error } = requireAuth(cookies)
        if (error) {
            return HttpResponse.json({ message: error }, { status: 401 })
        }

        // Return the BFF /auth/me shape: { id, email, role }
        return HttpResponse.json({
            id: user!.id,
            email: user!.email,
            role: user!.role,
        })
    }),

    // POST /auth/logout — clear the auth cookie
    http.post(`${env.API_URL}/auth/logout`, async () => {
        await networkDelay()

        Cookies.remove(AUTH_COOKIE)

        return HttpResponse.json(
            { message: 'Logged out' },
            {
                headers: {
                    'Set-Cookie': `${AUTH_COOKIE}=; Path=/; Max-Age=0`,
                },
            },
        )
    }),

    // POST /auth/refresh — mock a successful token refresh for tests
    http.post(`${env.API_URL}/auth/refresh`, async ({ cookies }) => {
        await networkDelay()

        const { error } = requireAuth(cookies)
        if (error) {
            return HttpResponse.json(
                { message: 'Unauthorized' },
                { status: 401 },
            )
        }

        return HttpResponse.json({ message: 'Token refreshed' })
    }),
]
