import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import {
    createDirectory,
    isDriveEntriesKey,
    type NewDirectory,
} from '@/api/domains/drive'

/**
 * The mutation key is the hook's own, not a listing's: two mutation hooks
 * sharing a key would share their `isMutating` state. Invalidation is explicit
 * and lives here, so no call site can forget it — and it covers every listing,
 * because a create, a move or a delete changes two levels of the tree at once.
 */
const MUTATION_KEY = ['drive', 'createDirectory'] as const

export function useApiCreateDirectory() {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        MUTATION_KEY,
        (_key, { arg }: { arg: NewDirectory }) => createDirectory(arg),
        { onSuccess: () => void mutate(isDriveEntriesKey) },
    )
}
