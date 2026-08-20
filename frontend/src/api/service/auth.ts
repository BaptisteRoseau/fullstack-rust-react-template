import useSWR from 'swr'

import {
    ME_ENDPOINT,
    LOGOUT_ENDPOINT,
    type CurrentUser,
    type UpdateProfileBody,
} from '@/api/auth'
import { apiFetch } from '@/api/client'
import { isApiError } from '@/api/errors'
import { useApiAction } from '@/api/utils/useApiAction'

async function fetchCurrentUser(path: string): Promise<CurrentUser | null> {
    try {
        return await apiFetch<CurrentUser>(path)
    } catch (error) {
        if (isApiError(error) && error.status === 401) {
            return null
        }
        throw error
    }
}

export function useCurrentUser() {
    return useSWR<CurrentUser | null>(ME_ENDPOINT, fetchCurrentUser)
}

export function useUpdateProfile() {
    return useApiAction<UpdateProfileBody, CurrentUser>(ME_ENDPOINT, 'PATCH')
}

export function useLogout() {
    return useApiAction<void, void>(LOGOUT_ENDPOINT, 'POST')
}
