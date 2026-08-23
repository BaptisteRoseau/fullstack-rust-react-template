import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { currentUserKeys } from '@/api/domains/currentUser'
import { logout } from '@/api/domains/session'

const MUTATION_KEY = ['session', 'logout'] as const

/**
 * Clears the cached profile outright instead of revalidating it: the session is
 * gone, so refetching it would only answer 401.
 */
export function useApiLogout() {
    const { mutate } = useSWRConfig()

    return useSWRMutation(MUTATION_KEY, () => logout(), {
        onSuccess: () =>
            void mutate(currentUserKeys.me, null, { revalidate: false }),
    })
}
