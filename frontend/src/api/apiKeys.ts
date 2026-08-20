export const API_KEYS_ENDPOINT = '/api/api-key'

export const apiKeyEndpoint = (apiKeyId: string) =>
    `${API_KEYS_ENDPOINT}/${apiKeyId}`

export type ApiKey = {
    id: string
    name: string
    permissions: string[]
    createdAt: string
}

export type CreatedApiKey = ApiKey & { key: string }

export type CreateApiKeyBody = Pick<ApiKey, 'name' | 'permissions'>

export const API_KEY_PERMISSIONS = ['read', 'write', 'admin'] as const

export type ApiKeyPermission = (typeof API_KEY_PERMISSIONS)[number]
