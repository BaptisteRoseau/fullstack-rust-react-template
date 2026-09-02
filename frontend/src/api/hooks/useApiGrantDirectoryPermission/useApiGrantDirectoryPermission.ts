import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import {
    driveKeys,
    grantDirectoryPermission,
    type NewPermissionGrant,
} from '@/api/domains/drive'

const mutationKey = (directoryId: string) =>
    ['drive', 'grantDirectoryPermission', directoryId] as const

export function useApiGrantDirectoryPermission(directoryId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        mutationKey(directoryId),
        (_key, { arg }: { arg: NewPermissionGrant }) =>
            grantDirectoryPermission(directoryId, arg),
        {
            onSuccess: () =>
                void mutate(driveKeys.directoryPermissions(directoryId)),
        },
    )
}
