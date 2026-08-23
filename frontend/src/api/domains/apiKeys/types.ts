/** The permissions a key may carry, and the source of {@link ApiKeyPermission}. */
export const API_KEY_PERMISSIONS = ['read', 'write', 'admin'] as const

export type ApiKeyPermission = (typeof API_KEY_PERMISSIONS)[number]

export type ApiKey = {
    id: string
    name: string
    permissions: ApiKeyPermission[]
    createdAt: Date
}

/** An API key as returned by its creation, the one time its secret is shown. */
export type CreatedApiKey = ApiKey & { secret: string }

export type NewApiKey = Pick<ApiKey, 'name' | 'permissions'>
