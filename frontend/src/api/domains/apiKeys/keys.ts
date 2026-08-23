export const apiKeyKeys = {
    all: ['apiKeys'] as const,
    detail: (apiKeyId: string) => ['apiKeys', apiKeyId] as const,
}
