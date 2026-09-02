import { useApiDeleteDirectory } from '@/api/hooks/useApiDeleteDirectory'
import { useApiDeleteFile } from '@/api/hooks/useApiDeleteFile'

import type { DriveEntryKind } from '../../types'

export type DriveEntryRemover = {
    trigger: () => Promise<void>
    isMutating: boolean
}

/** The delete of whichever half of the listing the entry came from. */
export function useDriveEntryDelete(
    kind: DriveEntryKind,
    entryId: string,
): DriveEntryRemover {
    const directory = useApiDeleteDirectory(entryId)
    const file = useApiDeleteFile(entryId)

    return kind === 'directory'
        ? {
              trigger: () => directory.trigger(),
              isMutating: directory.isMutating,
          }
        : { trigger: () => file.trigger(), isMutating: file.isMutating }
}
