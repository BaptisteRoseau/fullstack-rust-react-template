import { REFRESH_ENDPOINT } from '@/api/auth'
import { client } from '@/api/generated/client.gen'
import { env } from '@/config/env'

import { toApiError, toNetworkError } from './errors'

const API_BASE_URL = `${env.API_URL}/api`
const REFRESH_URL = `${env.API_URL}${REFRESH_ENDPOINT}`

let refreshPromise: Promise<boolean> | null = null

/**
 * Renews the session cookies, at most once at a time: a page that fires several
 * requests into an expired session must not send several refreshes.
 */
async function refreshSession(): Promise<boolean> {
    if (!refreshPromise) {
        refreshPromise = fetch(REFRESH_URL, {
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

/**
 * `fetch`, with an expired access token renewed once and the request replayed.
 *
 * Wrapping `fetch` rather than using the generated client's interceptors keeps
 * this independent of the code generator, and testable on its own. The retry
 * copy is taken before the first send because a request body is a stream that
 * cannot be read twice.
 */
export const fetchWithSessionRefresh: typeof fetch = async (input, init) => {
    const request = new Request(input, init)
    const retry = request.clone()

    const response = await fetch(request)
    if (response.status !== 401 || request.url === REFRESH_URL) {
        return response
    }

    return (await refreshSession()) ? fetch(retry) : response
}

client.setConfig({
    baseUrl: API_BASE_URL,
    credentials: 'include',
    fetch: fetchWithSessionRefresh,
})

type SdkResult<TData> = {
    data?: TData
    error?: unknown
    response?: Response
}

/**
 * Unwraps a generated SDK call: the payload on success, an `ApiError` on every
 * failure. Fetchers go through this so that no caller above `src/api` ever sees
 * the SDK's `{ data, error }` pair, or an error shape that depends on how the
 * request failed.
 *
 * The generated client catches everything it throws, so a result carrying no
 * `response` is how a request that never reached the server comes back.
 */
export async function apiCall<TData>(
    call: () => Promise<SdkResult<TData>>,
): Promise<TData> {
    let result: SdkResult<TData>
    try {
        result = await call()
    } catch (cause) {
        throw toNetworkError(cause)
    }

    if (result.error !== undefined || result.response?.ok === false) {
        throw result.response
            ? toApiError(result.error, result.response)
            : toNetworkError(result.error)
    }

    return result.data as TData
}

/**
 * @deprecated The hand-written transport, kept until every call site moves to a
 * domain fetcher. Use {@link apiCall} with a generated SDK function instead.
 */
export async function apiFetch<T>(
    path: string,
    init?: RequestInit,
): Promise<T> {
    const response = await fetchWithSessionRefresh(`${env.API_URL}${path}`, {
        credentials: 'include',
        ...init,
        headers: { 'Content-Type': 'application/json', ...init?.headers },
    })

    if (!response.ok) {
        const body = await response.json().catch(() => null)
        throw toApiError(body, response)
    }

    return response.status === 204 ? (undefined as T) : await response.json()
}
