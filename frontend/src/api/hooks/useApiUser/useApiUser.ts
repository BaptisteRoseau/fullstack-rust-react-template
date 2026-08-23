import useSWR from 'swr'

import { fetchUser, userKeys } from '@/api/domains/users'

export function useApiUser(userId: string | undefined) {
    return useSWR(userId ? userKeys.detail(userId) : null, ([, id]) =>
        fetchUser(id),
    )
}
