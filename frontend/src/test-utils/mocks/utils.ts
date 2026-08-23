import { delay } from 'msw'

export const SESSION_COOKIE = 'mock_session'

export const SET_SESSION_COOKIE = `${SESSION_COOKIE}=active; Path=/; SameSite=Lax`
export const CLEAR_SESSION_COOKIE = `${SESSION_COOKIE}=; Path=/; Max-Age=0`

export const FORWARDED_COOKIE_HEADER = 'x-forwarded-cookie'

/**
 * Absolute paths as the backend serves them, mirroring the OpenAPI document's
 * `/api` server prefix. Handlers derive their URLs from here so that a renamed
 * route breaks in one place.
 */
export const API_PATHS = {
    apiKeys: '/api/api-key',
    login: '/api/auth/login',
    logout: '/api/auth/logout',
    me: '/api/auth/me',
    refresh: '/api/auth/refresh',
    register: '/api/auth/register',
    users: '/api/user',
} as const

export function endpoint(path: string) {
    return `*${path}`
}

export async function networkDelay() {
    await delay(process.env.NODE_ENV === 'test' ? 0 : 120)
}

export function isAuthenticated(request: Request) {
    const header =
        request.headers.get(FORWARDED_COOKIE_HEADER) ??
        request.headers.get('cookie') ??
        ''

    return header
        .split(';')
        .some((cookie) => cookie.trim() === `${SESSION_COOKIE}=active`)
}
