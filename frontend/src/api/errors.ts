export class ApiError extends Error {
    constructor(
        message: string,
        readonly status: number,
        readonly body: unknown,
    ) {
        super(message)
        this.name = 'ApiError'
    }
}

export function isApiError(error: unknown): error is ApiError {
    return error instanceof ApiError
}

export function apiErrorMessage(error: unknown, fallback: string): string {
    if (!isApiError(error)) {
        return fallback
    }
    const body = error.body as { error?: string; message?: string } | null
    return body?.error ?? body?.message ?? fallback
}
