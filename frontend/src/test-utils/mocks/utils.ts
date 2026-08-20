import { delay } from 'msw'

export const SESSION_COOKIE = 'mock_session'

export const SET_SESSION_COOKIE = `${SESSION_COOKIE}=active; Path=/; SameSite=Lax`
export const CLEAR_SESSION_COOKIE = `${SESSION_COOKIE}=; Path=/; Max-Age=0`

export const FORWARDED_COOKIE_HEADER = 'x-forwarded-cookie'

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
