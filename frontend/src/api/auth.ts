import { env } from '@/config/env'

export const ME_ENDPOINT = '/api/auth/me'
export const LOGOUT_ENDPOINT = '/api/auth/logout'
export const REFRESH_ENDPOINT = '/api/auth/refresh'

export type CurrentUser = {
    id: string
    email: string
    firstName: string
    lastName: string
    role: string
    bio: string
    teamId: string
    createdAt: number
}

export type UpdateProfileBody = Pick<
    CurrentUser,
    'firstName' | 'lastName' | 'bio'
>

export function authRedirectUrl(
    screen: 'login' | 'register',
    redirectTo?: string,
): string {
    const query = redirectTo
        ? `?${new URLSearchParams({ redirect: redirectTo })}`
        : ''
    return `${env.API_URL}/api/auth/${screen}${query}`
}

export function fullName(user: CurrentUser): string {
    return `${user.firstName} ${user.lastName}`.trim() || user.email
}
