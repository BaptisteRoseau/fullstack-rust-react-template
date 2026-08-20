import useSWR from 'swr'

import {
    API_KEYS_ENDPOINT,
    apiKeyEndpoint,
    type ApiKey,
    type CreateApiKeyBody,
    type CreatedApiKey,
} from '@/api/apiKeys'
import { useApiAction } from '@/api/utils/useApiAction'

export function useApiKeys() {
    return useSWR<ApiKey[]>(API_KEYS_ENDPOINT)
}

export function useCreateApiKey() {
    return useApiAction<CreateApiKeyBody, CreatedApiKey>(
        API_KEYS_ENDPOINT,
        'POST',
    )
}

export function useRevokeApiKey(apiKeyId: string) {
    return useApiAction<void, void>(apiKeyEndpoint(apiKeyId), 'DELETE')
}
