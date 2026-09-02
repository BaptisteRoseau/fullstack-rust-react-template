import useSWR from 'swr'

import { driveKeys, fetchEntries } from '@/api/domains/drive'

/**
 * The listed directory is read back out of the cache key rather than captured
 * from the closure, so the key and the request it stands for cannot disagree.
 * `'root'` is the key's stand-in for the absent parent.
 */
export function useApiEntries(parentId?: string | null) {
    return useSWR(driveKeys.entries(parentId), ([, , directoryId]) =>
        fetchEntries(directoryId === 'root' ? null : directoryId),
    )
}
