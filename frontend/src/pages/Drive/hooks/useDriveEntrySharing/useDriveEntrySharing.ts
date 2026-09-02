import type { NewPermissionGrant, PermissionGrant } from '@/api/domains/drive'
import { useApiDirectoryPermissions } from '@/api/hooks/useApiDirectoryPermissions'
import { useApiFilePermissions } from '@/api/hooks/useApiFilePermissions'
import { useApiGrantDirectoryPermission } from '@/api/hooks/useApiGrantDirectoryPermission'
import { useApiGrantFilePermission } from '@/api/hooks/useApiGrantFilePermission'
import { useApiRevokeDirectoryPermission } from '@/api/hooks/useApiRevokeDirectoryPermission'
import { useApiRevokeFilePermission } from '@/api/hooks/useApiRevokeFilePermission'

import type { DriveEntryKind } from '../../types'

export type DriveEntrySharing = {
    grants: {
        data: PermissionGrant[] | undefined
        error: unknown
        isLoading: boolean
    }
    grant: {
        trigger: (grant: NewPermissionGrant) => Promise<PermissionGrant>
        isMutating: boolean
    }
    revoke: {
        trigger: (userId: string) => Promise<void>
        isMutating: boolean
    }
}

/**
 * The three sharing operations for one entry, whichever half of the listing it
 * came from. `isOpen` gates the read: a share dialog that has never been opened
 * must not fetch grants for every card on the page.
 */
export function useDriveEntrySharing(
    kind: DriveEntryKind,
    entryId: string,
    isOpen: boolean,
): DriveEntrySharing {
    const isDirectory = kind === 'directory'
    const readId = isOpen ? entryId : undefined

    const directoryGrants = useApiDirectoryPermissions(
        isDirectory ? readId : undefined,
    )
    const fileGrants = useApiFilePermissions(isDirectory ? undefined : readId)
    const grantDirectory = useApiGrantDirectoryPermission(entryId)
    const grantFile = useApiGrantFilePermission(entryId)
    const revokeDirectory = useApiRevokeDirectoryPermission(entryId)
    const revokeFile = useApiRevokeFilePermission(entryId)

    const grants = isDirectory ? directoryGrants : fileGrants

    return {
        grants: {
            data: grants.data,
            error: grants.error,
            isLoading: grants.isLoading,
        },
        grant: isDirectory
            ? {
                  trigger: (grant) => grantDirectory.trigger(grant),
                  isMutating: grantDirectory.isMutating,
              }
            : {
                  trigger: (grant) => grantFile.trigger(grant),
                  isMutating: grantFile.isMutating,
              },
        revoke: isDirectory
            ? {
                  trigger: (userId) => revokeDirectory.trigger(userId),
                  isMutating: revokeDirectory.isMutating,
              }
            : {
                  trigger: (userId) => revokeFile.trigger(userId),
                  isMutating: revokeFile.isMutating,
              },
    }
}
