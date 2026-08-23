import type { GetMeResponse, PatchMeRequest } from '@/api/generated'

import type { CurrentUser, ProfileUpdate } from './types'

/** `createdAt` is a Unix timestamp in milliseconds on this endpoint. */
export function fromGetMeResponse(response: GetMeResponse): CurrentUser {
    return {
        id: response.id,
        email: response.email,
        firstName: response.firstName,
        lastName: response.lastName,
        role: response.role,
        teamId: response.teamId,
        createdAt: new Date(response.createdAt),
    }
}

export function toPatchMeRequest(profile: ProfileUpdate): PatchMeRequest {
    return { firstName: profile.firstName, lastName: profile.lastName }
}
