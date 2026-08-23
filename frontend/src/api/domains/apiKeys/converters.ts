import type {
    CreateApiKeyRequest,
    CreateApiKeyResponse,
    GetApiKeyResponse,
} from '@/api/generated'

import {
    API_KEY_PERMISSIONS,
    type ApiKey,
    type ApiKeyPermission,
    type CreatedApiKey,
    type NewApiKey,
} from './types'

const isApiKeyPermission = (value: string): value is ApiKeyPermission =>
    (API_KEY_PERMISSIONS as readonly string[]).includes(value)

/**
 * Unknown permissions are dropped rather than rejected: a permission the
 * backend gains tomorrow must not blank out today's table.
 */
export function fromGetApiKeyResponse(response: GetApiKeyResponse): ApiKey {
    return {
        id: response.id,
        name: response.name,
        permissions: response.permissions.filter(isApiKeyPermission),
        createdAt: new Date(response.createdAt),
    }
}

export function fromCreateApiKeyResponse(
    response: CreateApiKeyResponse,
): CreatedApiKey {
    return { ...fromGetApiKeyResponse(response), secret: response.key }
}

export function toCreateApiKeyRequest(apiKey: NewApiKey): CreateApiKeyRequest {
    return { name: apiKey.name, permissions: [...apiKey.permissions] }
}
