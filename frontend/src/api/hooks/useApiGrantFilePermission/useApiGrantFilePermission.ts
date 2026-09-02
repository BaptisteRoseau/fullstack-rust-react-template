import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import {
    driveKeys,
    grantFilePermission,
    type NewPermissionGrant,
} from '@/api/domains/drive'

const mutationKey = (fileId: string) =>
    ['drive', 'grantFilePermission', fileId] as const

export function useApiGrantFilePermission(fileId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        mutationKey(fileId),
        (_key, { arg }: { arg: NewPermissionGrant }) =>
            grantFilePermission(fileId, arg),
        { onSuccess: () => void mutate(driveKeys.filePermissions(fileId)) },
    )
}
