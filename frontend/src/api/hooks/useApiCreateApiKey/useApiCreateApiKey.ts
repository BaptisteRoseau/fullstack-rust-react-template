import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { apiKeyKeys, createApiKey, type NewApiKey } from '@/api/domains/apiKeys'

/**
 * The mutation key is the hook's own, not the list's: two mutation hooks
 * sharing a key would share their `isMutating` state. Invalidation is explicit
 * and lives here, so no call site can forget it.
 */
const MUTATION_KEY = ['apiKeys', 'create'] as const

export function useApiCreateApiKey() {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        MUTATION_KEY,
        (_key, { arg }: { arg: NewApiKey }) => createApiKey(arg),
        { onSuccess: () => void mutate(apiKeyKeys.all) },
    )
}
