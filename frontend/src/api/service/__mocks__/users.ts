import { vi } from 'vitest'

export const useUser = vi.fn().mockReturnValue({
    data: undefined,
    error: undefined,
    isLoading: false,
    mutate: vi.fn(),
})
