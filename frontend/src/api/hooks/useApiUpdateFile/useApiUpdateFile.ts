import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import {
    type DriveEntryUpdate,
    isDriveEntriesKey,
    updateFile,
} from '@/api/domains/drive'

const mutationKey = (fileId: string) => ['drive', 'updateFile', fileId] as const

export function useApiUpdateFile(fileId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        mutationKey(fileId),
        (_key, { arg }: { arg: DriveEntryUpdate }) => updateFile(fileId, arg),
        { onSuccess: () => void mutate(isDriveEntriesKey) },
    )
}
