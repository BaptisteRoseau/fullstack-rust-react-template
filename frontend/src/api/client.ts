import { REFRESH_ENDPOINT } from '@/api/auth'
import { env } from '@/config/env'

import { ApiError } from './errors'

let refreshPromise: Promise<boolean> | null = null

async function refreshSession(): Promise<boolean> {
    if (!refreshPromise) {
        refreshPromise = fetch(`${env.API_URL}${REFRESH_ENDPOINT}`, {
            method: 'POST',
            credentials: 'include',
        })
            .then((response) => response.ok)
            .catch(() => false)
            .finally(() => {
                refreshPromise = null
            })
    }
    return refreshPromise
}

async function request(path: string, init?: RequestInit): Promise<Response> {
    return fetch(`${env.API_URL}${path}`, {
        credentials: 'include',
        ...init,
        headers: {
            'Content-Type': 'application/json',
            ...init?.headers,
        },
    })
}

export async function apiFetch<T>(
    path: string,
    init?: RequestInit,
): Promise<T> {
    let response = await request(path, init)

    if (response.status === 401 && path !== REFRESH_ENDPOINT) {
        if (await refreshSession()) {
            response = await request(path, init)
        }
    }

    if (!response.ok) {
        const body = await response.json().catch(() => null)
        throw new ApiError(
            `${init?.method ?? 'GET'} ${path} failed`,
            response.status,
            body,
        )
    }

    if (response.status === 204) {
        return undefined as T
    }

    return (await response.json()) as T
}
