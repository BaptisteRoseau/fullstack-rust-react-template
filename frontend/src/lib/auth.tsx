import { isAxiosError } from 'axios'
import { configureAuth } from 'react-query-auth'
import { Navigate, useLocation } from 'react-router'

import { env } from '@/config/env'
import { paths } from '@/config/paths'
import { User } from '@/types/api'

import { api } from './api-client'

// Auth is delegated to Keycloak through the backend Backend-for-Frontend (BFF):
// the browser is redirected to the backend, which drives the OAuth Authorization
// Code + PKCE flow and stores the tokens in httpOnly cookies. The frontend never
// sees the tokens; it only reads the current user and triggers logout.

const getUser = async (): Promise<User | null> => {
    try {
        // The response interceptor already unwraps to the HTTP body, which is the user.
        return await api.get('/api/auth/me')
    } catch (error) {
        // Being logged out is a normal state, not a failure: resolve to "no user"
        // so the app renders its public view. The interceptor has already tried a
        // silent token refresh by the time a 401 reaches us. Any other failure is
        // genuine and stays an error (the interceptor toasts it).
        if (isAxiosError(error) && error.response?.status === 401) {
            return null
        }
        throw error
    }
}

const logout = (): Promise<void> => {
    return api.post('/api/auth/logout')
}

// Entry-point of the BFF flow. The backend redirects the browser to Keycloak's
// hosted login or registration page and handles the callback.
const authEntrypoint = (
    screen: 'login' | 'register',
    redirectTo?: string | null,
) => {
    const params = new URLSearchParams()
    if (redirectTo) params.set('redirect', redirectTo)
    const query = params.toString()
    return `${env.API_URL}/api/auth/${screen}${query ? `?${query}` : ''}`
}

export const loginUrl = (redirectTo?: string | null) =>
    authEntrypoint('login', redirectTo)

export const registerUrl = (redirectTo?: string | null) =>
    authEntrypoint('register', redirectTo)

const authConfig = {
    userFn: getUser,
    // Login and registration happen via a full-page redirect to the backend, so
    // these resolve into navigation rather than returning a user.
    loginFn: () => {
        window.location.href = loginUrl()
        return new Promise<User>(() => {})
    },
    registerFn: () => {
        window.location.href = registerUrl()
        return new Promise<User>(() => {})
    },
    logoutFn: logout,
}

export const { useUser, useLogin, useLogout, useRegister, AuthLoader } =
    configureAuth(authConfig)

export const ProtectedRoute = ({ children }: { children: React.ReactNode }) => {
    const user = useUser()
    const location = useLocation()

    if (!user.data) {
        return (
            <Navigate
                to={paths.auth.login.getHref(location.pathname)}
                replace
            />
        )
    }

    return children
}
