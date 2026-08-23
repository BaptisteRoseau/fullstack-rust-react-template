import { randEmail, randFirstName, randLastName, randUuid } from '@ngneat/falso'

import type { CurrentUser } from '@/api/domains/currentUser'
import type { GetMeResponse } from '@/api/generated'

export function buildCurrentUser(
    overrides: Partial<CurrentUser> = {},
): CurrentUser {
    return {
        id: randUuid(),
        email: randEmail(),
        firstName: randFirstName(),
        lastName: randLastName(),
        role: 'user',
        teamId: randUuid(),
        createdAt: new Date(),
        ...overrides,
    }
}

/**
 * The wire shape, which is not the domain shape: `createdAt` is a Unix
 * timestamp in milliseconds here and a `Date` in {@link buildCurrentUser}.
 */
export function buildGetMeResponse(
    overrides: Partial<GetMeResponse> = {},
): GetMeResponse {
    return {
        id: randUuid(),
        email: randEmail(),
        firstName: randFirstName(),
        lastName: randLastName(),
        role: 'user',
        teamId: randUuid(),
        createdAt: Date.now(),
        ...overrides,
    }
}
