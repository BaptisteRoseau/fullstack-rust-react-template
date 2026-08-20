import { randEmail, randFirstName, randLastName, randUuid } from '@ngneat/falso'

import type { CurrentUser } from '@/api/auth'

export function buildCurrentUser(
    overrides: Partial<CurrentUser> = {},
): CurrentUser {
    return {
        id: randUuid(),
        email: randEmail(),
        firstName: randFirstName(),
        lastName: randLastName(),
        role: 'user',
        bio: '',
        teamId: randUuid(),
        createdAt: Date.now(),
        ...overrides,
    }
}
