import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { driveKeys, revokeDirectoryPermission } from '@/api/domains/drive'

const mutationKey = (directoryId: string) =>
    ['drive', 'revokeDirectoryPermission', directoryId] as const

export function useApiRevokeDirectoryPermission(directoryId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        mutationKey(directoryId),
        (_key, { arg }: { arg: string }) =>
            revokeDirectoryPermission(directoryId, arg),
        {
            onSuccess: () =>
                void mutate(driveKeys.directoryPermissions(directoryId)),
        },
    )
}
