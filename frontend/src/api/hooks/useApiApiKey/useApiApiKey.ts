import useSWR from 'swr'

import { apiKeyKeys, fetchApiKey } from '@/api/domains/apiKeys'

/**
 * The argument is read back out of the cache key rather than captured from the
 * closure, so the key and the request it stands for cannot disagree.
 */
export function useApiApiKey(apiKeyId: string | undefined) {
    return useSWR(apiKeyId ? apiKeyKeys.detail(apiKeyId) : null, ([, id]) =>
        fetchApiKey(id),
    )
}
