import { randProductName, randUuid } from '@ngneat/falso'

import type { ApiKey, CreatedApiKey } from '@/api/apiKeys'

export function buildApiKey(overrides: Partial<ApiKey> = {}): ApiKey {
    return {
        id: randUuid(),
        name: randProductName(),
        permissions: ['read'],
        createdAt: new Date().toISOString(),
        ...overrides,
    }
}

export function buildCreatedApiKey(
    overrides: Partial<CreatedApiKey> = {},
): CreatedApiKey {
    return { ...buildApiKey(), key: `sk_${randUuid()}`, ...overrides }
}
