import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { driveKeys, revokeFilePermission } from '@/api/domains/drive'

const mutationKey = (fileId: string) =>
    ['drive', 'revokeFilePermission', fileId] as const

export function useApiRevokeFilePermission(fileId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        mutationKey(fileId),
        (_key, { arg }: { arg: string }) => revokeFilePermission(fileId, arg),
        { onSuccess: () => void mutate(driveKeys.filePermissions(fileId)) },
    )
}
