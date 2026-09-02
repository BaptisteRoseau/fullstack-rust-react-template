import type {
    DriveDirectory,
    DriveEntryUpdate,
    DriveFile,
} from '@/api/domains/drive'
import { useApiUpdateDirectory } from '@/api/hooks/useApiUpdateDirectory'
import { useApiUpdateFile } from '@/api/hooks/useApiUpdateFile'

import type { DriveEntryKind } from '../../types'

export type DriveEntryUpdater = {
    trigger: (update: DriveEntryUpdate) => Promise<DriveDirectory | DriveFile>
    isMutating: boolean
}

/**
 * Renaming and moving are one operation on two endpoints. Both bindings are
 * created because hooks cannot be called conditionally; neither issues a
 * request until it is triggered, so the unused one costs nothing. The pair is
 * rewrapped rather than returned as a union so that call sites see one
 * signature instead of two that TypeScript refuses to merge.
 */
export function useDriveEntryUpdate(
    kind: DriveEntryKind,
    entryId: string,
): DriveEntryUpdater {
    const directory = useApiUpdateDirectory(entryId)
    const file = useApiUpdateFile(entryId)

    return kind === 'directory'
        ? {
              trigger: (update) => directory.trigger(update),
              isMutating: directory.isMutating,
          }
        : {
              trigger: (update) => file.trigger(update),
              isMutating: file.isMutating,
          }
}
