import { apiCall } from '@/api/client'
import { logout as logoutRequest } from '@/api/generated'
import { env } from '@/config/env'

export async function logout(): Promise<void> {
    await apiCall(() => logoutRequest())
}

/**
 * The OIDC entry points are browser navigations, not fetches: the backend
 * answers them with a redirect to the identity provider, so a call site sets
 * `location.href` rather than awaiting a response.
 */
function authRedirectUrl(screen: 'login' | 'register', redirectTo?: string) {
    const query = redirectTo
        ? `?${new URLSearchParams({ redirect: redirectTo })}`
        : ''
    return `${env.API_URL}/api/auth/${screen}${query}`
}

export const loginUrl = (redirectTo?: string) =>
    authRedirectUrl('login', redirectTo)

export const registerUrl = (redirectTo?: string) =>
    authRedirectUrl('register', redirectTo)
