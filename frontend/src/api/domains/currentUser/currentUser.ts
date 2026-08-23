import { apiCall } from '@/api/client'
import { isApiError } from '@/api/errors'
import { me, updateMe } from '@/api/generated'

import { fromGetMeResponse, toPatchMeRequest } from './converters'
import type { CurrentUser, ProfileUpdate } from './types'

/**
 * The signed-in user, or `null` when nobody is signed in.
 *
 * A 401 here is an answer, not a failure: it is how the backend says the
 * session is gone, and every caller renders the signed-out interface for it.
 */
export async function fetchCurrentUser(): Promise<CurrentUser | null> {
    try {
        return fromGetMeResponse(await apiCall(() => me()))
    } catch (error) {
        if (isApiError(error) && error.status === 401) {
            return null
        }
        throw error
    }
}

export async function updateCurrentUser(
    profile: ProfileUpdate,
): Promise<CurrentUser> {
    return fromGetMeResponse(
        await apiCall(() => updateMe({ body: toPatchMeRequest(profile) })),
    )
}
