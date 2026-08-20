import { vi } from 'vitest'

export const useApiKeys = vi.fn().mockReturnValue({
    data: [],
    error: undefined,
    isLoading: false,
    mutate: vi.fn(),
})

export const useCreateApiKey = vi.fn().mockReturnValue({
    trigger: vi.fn(),
    isMutating: false,
})

export const useRevokeApiKey = vi.fn().mockReturnValue({
    trigger: vi.fn(),
    isMutating: false,
})
