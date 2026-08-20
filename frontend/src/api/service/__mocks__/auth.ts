import { vi } from 'vitest'

export const useCurrentUser = vi.fn().mockReturnValue({
    data: null,
    error: undefined,
    isLoading: false,
    mutate: vi.fn(),
})

export const useUpdateProfile = vi.fn().mockReturnValue({
    trigger: vi.fn(),
    isMutating: false,
})

export const useLogout = vi.fn().mockReturnValue({
    trigger: vi.fn(),
    isMutating: false,
})
