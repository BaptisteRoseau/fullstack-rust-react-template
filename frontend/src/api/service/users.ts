import useSWR from 'swr'

import { userEndpoint, type UserInfo } from '@/api/users'

export function useUser(userId: string | undefined) {
    return useSWR<UserInfo>(userId ? userEndpoint(userId) : null)
}
