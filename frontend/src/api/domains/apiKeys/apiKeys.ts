import { apiCall } from '@/api/client'
import {
    createApiKey as createApiKeyRequest,
    deleteApiKey,
    getApiKey,
    listApiKeys,
} from '@/api/generated'

import {
    fromCreateApiKeyResponse,
    fromGetApiKeyResponse,
    toCreateApiKeyRequest,
} from './converters'
import type { ApiKey, CreatedApiKey, NewApiKey } from './types'

export async function fetchApiKeys(): Promise<ApiKey[]> {
    const response = await apiCall(() => listApiKeys())
    return response.map(fromGetApiKeyResponse)
}

export async function fetchApiKey(apiKeyId: string): Promise<ApiKey> {
    return fromGetApiKeyResponse(
        await apiCall(() => getApiKey({ path: { id: apiKeyId } })),
    )
}

export async function createApiKey(apiKey: NewApiKey): Promise<CreatedApiKey> {
    return fromCreateApiKeyResponse(
        await apiCall(() =>
            createApiKeyRequest({ body: toCreateApiKeyRequest(apiKey) }),
        ),
    )
}

export async function revokeApiKey(apiKeyId: string): Promise<void> {
    await apiCall(() => deleteApiKey({ path: { id: apiKeyId } }))
}
