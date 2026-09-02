import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import {
    type DriveEntryUpdate,
    isDriveEntriesKey,
    updateDirectory,
} from '@/api/domains/drive'

const mutationKey = (directoryId: string) =>
    ['drive', 'updateDirectory', directoryId] as const

export function useApiUpdateDirectory(directoryId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        mutationKey(directoryId),
        (_key, { arg }: { arg: DriveEntryUpdate }) =>
            updateDirectory(directoryId, arg),
        { onSuccess: () => void mutate(isDriveEntriesKey) },
    )
}
