import useSWR from 'swr'

import { apiKeyKeys, fetchApiKeys } from '@/api/domains/apiKeys'

export function useApiApiKeys() {
    return useSWR(apiKeyKeys.all, () => fetchApiKeys())
}
