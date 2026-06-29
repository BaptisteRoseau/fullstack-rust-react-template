import Axios, {
    AxiosError,
    AxiosRequestConfig,
    InternalAxiosRequestConfig,
} from 'axios'

import { useNotifications } from '@/components/ui/notifications'
import { env } from '@/config/env'

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

// A single in-flight refresh is shared across concurrent 401s so we hit
// `/auth/refresh` only once and let every pending request retry afterwards.
let refreshPromise: Promise<unknown> | null = null

function refreshSession() {
    if (!refreshPromise) {
        refreshPromise = api.post('/auth/refresh').finally(() => {
            refreshPromise = null
        })
    }
    return refreshPromise
}

api.interceptors.response.use(
    (response) => {
        return response.data
    },
    async (error: AxiosError) => {
        const config = error.config as
            | (InternalAxiosRequestConfig & { _retry?: boolean })
            | undefined
        const status = error.response?.status
        const url = config?.url ?? ''
        const isAuthFlow =
            url.includes('/auth/refresh') || url.includes('/auth/login')

        // On an expired/invalid access token, silently refresh once and replay the
        // original request. The httpOnly refresh cookie is sent automatically.
        if (status === 401 && config && !config._retry && !isAuthFlow) {
            config._retry = true
            try {
                await refreshSession()
                return api(config as AxiosRequestConfig)
            } catch {
                // Refresh failed: the user is logged out. Let ProtectedRoute handle
                // navigation rather than forcing a full-page redirect here.
                return Promise.reject(error)
            }
        }

        // 401s are an expected "logged out" signal; don't surface them as toasts.
        if (status !== 401) {
            const data = error.response?.data as
                | { error?: string; message?: string }
                | undefined
            const message = data?.error || data?.message || error.message
            useNotifications.getState().addNotification({
                type: 'error',
                title: 'Error',
                message,
            })
        }

        return Promise.reject(error)
    },
)
