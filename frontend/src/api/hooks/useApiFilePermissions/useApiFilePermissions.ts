import useSWR from 'swr'

import { driveKeys, fetchFilePermissions } from '@/api/domains/drive'

export function useApiFilePermissions(fileId: string | undefined) {
    return useSWR(
        fileId ? driveKeys.filePermissions(fileId) : null,
        ([, , id]) => fetchFilePermissions(id),
    )
}
