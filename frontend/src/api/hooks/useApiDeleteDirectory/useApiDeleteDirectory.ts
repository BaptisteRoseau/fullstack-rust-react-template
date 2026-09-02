import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { deleteDirectory, isDriveEntriesKey } from '@/api/domains/drive'

const mutationKey = (directoryId: string) =>
    ['drive', 'deleteDirectory', directoryId] as const

export function useApiDeleteDirectory(directoryId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        mutationKey(directoryId),
        () => deleteDirectory(directoryId),
        { onSuccess: () => void mutate(isDriveEntriesKey) },
    )
}
