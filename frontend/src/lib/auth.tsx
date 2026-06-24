import { configureAuth } from 'react-query-auth'
import { useLocation } from 'react-router'

import { env } from '@/config/env'
import { paths } from '@/config/paths'
import { User } from '@/types/api'

import { api } from './api-client'

// Derive the backend origin from API_URL (strip any trailing path segments).
// e.g. "http://localhost:8080" stays "http://localhost:8080"
const getApiOrigin = (): string => {
    try {
        const url = new URL(env.API_URL)
        return url.origin
    } catch {
        return env.API_URL
    }
}

const getUser = async (): Promise<User> => {
    return api.get('/auth/me')
}

// Redirect the browser to the backend OIDC login endpoint.
// The backend owns the OIDC flow and will set an HttpOnly cookie on completion.
export const login = (redirectTo?: string): void => {
    const origin = getApiOrigin()
    const redirect = encodeURIComponent(redirectTo ?? window.location.pathname)
    window.location.href = `${origin}/auth/login?redirect=${redirect}`
}

const logoutFn = async (): Promise<void> => {
    await api.post('/auth/logout')
    window.location.href = paths.home.getHref()
}

const authConfig = {
    userFn: getUser,
    // loginFn / registerFn are not used — login is a browser redirect.
    // react-query-auth requires them; provide no-ops that satisfy the type.
    loginFn: async (): Promise<User> => {
        throw new Error('Use login() redirect instead')
    },
    registerFn: async (): Promise<User> => {
        throw new Error('Registration is handled by Keycloak')
    },
    logoutFn,
}

export const { useUser, useLogin, useLogout, useRegister, AuthLoader } =
    configureAuth(authConfig)

export const ProtectedRoute = ({ children }: { children: React.ReactNode }) => {
    const user = useUser()
    const location = useLocation()

    if (!user.data) {
        // Trigger the backend OIDC redirect rather than navigating to a local page.
        login(location.pathname)
        return null
    }

    return children
}
