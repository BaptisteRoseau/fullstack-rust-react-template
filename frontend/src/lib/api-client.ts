import Axios, { AxiosError, InternalAxiosRequestConfig } from 'axios'

import { useNotifications } from '@/components/ui/notifications'
import { env } from '@/config/env'

// Custom header used internally to mark a request as already retried after a
// token refresh, preventing infinite retry loops.
const AUTH_RETRY_HEADER = 'x-auth-retried'

// Augment Axios internal config so we can track the retry flag on the config object.
declare module 'axios' {
    interface InternalAxiosRequestConfig {
        _authRetried?: boolean
    }
}

function authRequestInterceptor(config: InternalAxiosRequestConfig) {
    if (config.headers) {
        config.headers.Accept = 'application/json'
    }

    config.withCredentials = true
    return config
}

export const api = Axios.create({
    baseURL: env.API_URL,
})

api.interceptors.request.use(authRequestInterceptor)

api.interceptors.response.use(
    (response) => {
        return response.data
    },
    async (error: AxiosError) => {
        const message = error.response?.data
            ? (error.response.data as { message?: string }).message
            : error.message
        const config = error.config

        if (
            error.response?.status === 401 &&
            config &&
            !config._authRetried &&
            // Never attempt to refresh the refresh call or /auth/me to avoid loops.
            !config.url?.includes('/auth/refresh') &&
            !config.url?.includes('/auth/me')
        ) {
            config._authRetried = true
            try {
                // Attempt a silent token refresh. Mark as retried via header so
                // the request interceptor can tag InternalAxiosRequestConfig.
                await api.post('/auth/refresh', undefined, {
                    headers: { [AUTH_RETRY_HEADER]: '1' },
                })
                // Retry the original request with the refreshed cookie.
                return api(config)
            } catch {
                // Refresh failed — send the user to the backend login.
                const origin = (() => {
                    try {
                        return new URL(env.API_URL).origin
                    } catch {
                        return env.API_URL
                    }
                })()
                const redirect = encodeURIComponent(window.location.pathname)
                window.location.href = `${origin}/auth/login?redirect=${redirect}`
                return Promise.reject(error)
            }
        }

        useNotifications.getState().addNotification({
            type: 'error',
            title: 'Error',
            message: message ?? 'An error occurred',
        })

        return Promise.reject(error)
    },
)
