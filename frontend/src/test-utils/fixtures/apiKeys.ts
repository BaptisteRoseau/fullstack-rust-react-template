import { randProductName, randUuid } from '@ngneat/falso'

import type { ApiKey, CreatedApiKey } from '@/api/domains/apiKeys'
import type { CreateApiKeyResponse, GetApiKeyResponse } from '@/api/generated'

export function buildApiKey(overrides: Partial<ApiKey> = {}): ApiKey {
    return {
        id: randUuid(),
        name: randProductName(),
        permissions: ['read'],
        createdAt: new Date(),
        ...overrides,
    }
}

export function buildCreatedApiKey(
    overrides: Partial<CreatedApiKey> = {},
): CreatedApiKey {
    return { ...buildApiKey(), secret: `sk_${randUuid()}`, ...overrides }
}

/**
 * The wire shape, which is not the domain shape: `createdAt` is an RFC 3339
 * string here and a `Date` in {@link buildApiKey}, and the secret is `key`.
 */
export function buildGetApiKeyResponse(
    overrides: Partial<GetApiKeyResponse> = {},
): GetApiKeyResponse {
    return {
        id: randUuid(),
        name: randProductName(),
        permissions: ['read'],
        createdAt: new Date().toISOString(),
        ...overrides,
    }
}

export function buildCreateApiKeyResponse(
    overrides: Partial<CreateApiKeyResponse> = {},
): CreateApiKeyResponse {
    return {
        ...buildGetApiKeyResponse(),
        key: `sk_${randUuid()}`,
        ...overrides,
    }
}
