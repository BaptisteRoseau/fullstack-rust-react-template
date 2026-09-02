import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { deleteFile, isDriveEntriesKey } from '@/api/domains/drive'

const mutationKey = (fileId: string) => ['drive', 'deleteFile', fileId] as const

export function useApiDeleteFile(fileId: string) {
    const { mutate } = useSWRConfig()

    return useSWRMutation(mutationKey(fileId), () => deleteFile(fileId), {
        onSuccess: () => void mutate(isDriveEntriesKey),
    })
}
