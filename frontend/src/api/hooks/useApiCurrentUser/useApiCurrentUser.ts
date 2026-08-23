import useSWR from 'swr'

import { currentUserKeys, fetchCurrentUser } from '@/api/domains/currentUser'

export function useApiCurrentUser() {
    return useSWR(currentUserKeys.me, () => fetchCurrentUser())
}
