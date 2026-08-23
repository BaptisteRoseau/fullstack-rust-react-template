import { apiCall } from '@/api/client'
import { getUser } from '@/api/generated'

import { fromGetUserResponse } from './converters'
import type { User } from './types'

export async function fetchUser(userId: string): Promise<User> {
    return fromGetUserResponse(
        await apiCall(() => getUser({ path: { uuid: userId } })),
    )
}
