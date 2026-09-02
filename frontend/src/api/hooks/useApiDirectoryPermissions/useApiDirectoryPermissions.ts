import useSWR from 'swr'

import { driveKeys, fetchDirectoryPermissions } from '@/api/domains/drive'

export function useApiDirectoryPermissions(directoryId: string | undefined) {
    return useSWR(
        directoryId ? driveKeys.directoryPermissions(directoryId) : null,
        ([, , id]) => fetchDirectoryPermissions(id),
    )
}
