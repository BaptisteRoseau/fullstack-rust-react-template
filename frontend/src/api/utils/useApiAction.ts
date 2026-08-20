import useSWRMutation from 'swr/mutation'

import { apiFetch } from '@/api/client'
import type { ApiError } from '@/api/errors'

type Method = 'POST' | 'PUT' | 'PATCH' | 'DELETE'

export function useApiAction<TBody, TResult>(
    path: string,
    method: Method = 'POST',
) {
    return useSWRMutation<TResult, ApiError, string, TBody>(
        path,
        (url, { arg }) =>
            apiFetch<TResult>(url, {
                method,
                body: arg === undefined ? undefined : JSON.stringify(arg),
            }),
    )
}
