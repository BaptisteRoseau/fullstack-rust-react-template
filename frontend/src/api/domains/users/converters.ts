import type { GetUserResponse } from '@/api/generated'

import type { User } from './types'

export function fromGetUserResponse(response: GetUserResponse): User {
    return { name: response.name }
}
