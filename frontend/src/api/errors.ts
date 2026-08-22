import { useLingui } from '@lingui/react/macro'
import { useCallback, useMemo } from 'react'
import * as z from 'zod'

import type { ApiErrorId, ApiErrorResponse } from '@/api/generated'

/**
 * Failure causes the backend cannot report, because the request never produced
 * a documented response: `NETWORK` when `fetch` itself rejected, `PARSE` when
 * the body did not match [`ApiErrorResponse`].
 */
export const LOCAL_API_ERROR_IDS = ['NETWORK', 'PARSE'] as const

export type LocalApiErrorId = (typeof LOCAL_API_ERROR_IDS)[number]

export type AnyApiErrorId = ApiErrorId | LocalApiErrorId

const API_ERROR_IDS = [
    'UNEXPECTED',
    'UNAUTHORIZED',
    'FORBIDDEN',
    'TOKEN_EXPIRED',
    'NOT_FOUND',
    'TOO_MANY_REQUESTS',
    'HEADER_INVALID_ASCII_CHARACTERS',
] as const satisfies readonly ApiErrorId[]

const apiErrorResponseSchema = z.object({
    error: z.string(),
    id: z.enum(API_ERROR_IDS),
}) satisfies z.ZodType<ApiErrorResponse>

/** Every failure the API layer raises, whatever its cause. */
export class ApiError extends Error {
    constructor(
        message: string,
        readonly status: number,
        readonly id: AnyApiErrorId,
        readonly body: unknown,
    ) {
        super(message)
        this.name = 'ApiError'
    }
}

export function isApiError(error: unknown): error is ApiError {
    return error instanceof ApiError
}

/**
 * Builds an [`ApiError`] from a failed response body. A body the backend's
 * error contract does not describe yields `PARSE` rather than a guess.
 */
export function toApiError(body: unknown, response?: Response): ApiError {
    const status = response?.status ?? 0
    const parsed = apiErrorResponseSchema.safeParse(body)

    if (!parsed.success) {
        return new ApiError(
            `The request failed with status ${status}`,
            status,
            'PARSE',
            body,
        )
    }
    return new ApiError(parsed.data.error, status, parsed.data.id, body)
}

/** Builds an [`ApiError`] from a `fetch` rejection: offline, DNS, CORS. */
export function toNetworkError(cause: unknown): ApiError {
    const message =
        cause instanceof Error ? cause.message : 'The request could not be sent'
    return new ApiError(message, 0, 'NETWORK', cause)
}

type ApiErrorHandlers<T> = Partial<
    Record<AnyApiErrorId, (error: ApiError) => T>
> & {
    default: (error: unknown) => T
}

/**
 * Branches on why a call failed rather than on its status code, so a call site
 * never has to know that "expired session" and "not signed in" share a 401.
 */
export function matchApiError<T>(
    error: unknown,
    handlers: ApiErrorHandlers<T>,
): T {
    if (isApiError(error)) {
        const handler = handlers[error.id]
        if (handler) {
            return handler(error)
        }
    }
    return handlers.default(error)
}

/**
 * Turns a failure into a message meant for the user, in their language.
 *
 * `fallback` covers errors that never reached the API layer; the backend's own
 * English `error` string is for logs, never for the interface.
 */
export function useApiErrorMessage() {
    const { t } = useLingui()

    const messages = useMemo<Record<AnyApiErrorId, string>>(
        () => ({
            UNEXPECTED: t`Something went wrong. Please try again.`,
            UNAUTHORIZED: t`You need to be signed in to do that.`,
            FORBIDDEN: t`You are not allowed to do that.`,
            TOKEN_EXPIRED: t`Your session has expired. Please sign in again.`,
            NOT_FOUND: t`This resource no longer exists.`,
            TOO_MANY_REQUESTS: t`Too many requests. Please wait a moment and try again.`,
            HEADER_INVALID_ASCII_CHARACTERS: t`The request contained characters the server cannot read.`,
            NETWORK: t`The server could not be reached. Check your connection.`,
            PARSE: t`The server sent a response the application cannot read.`,
        }),
        [t],
    )

    return useCallback(
        (error: unknown, fallback?: string): string =>
            isApiError(error)
                ? messages[error.id]
                : (fallback ?? messages.UNEXPECTED),
        [messages],
    )
}

/**
 * @deprecated Surfaces the backend's untranslated string. Use
 * {@link useApiErrorMessage}; removed once every call site is migrated.
 */
export function apiErrorMessage(error: unknown, fallback: string): string {
    if (!isApiError(error)) {
        return fallback
    }
    const body = error.body as { error?: string; message?: string } | null
    return body?.error ?? body?.message ?? fallback
}
