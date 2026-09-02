import { useSWRConfig } from 'swr'
import useSWRMutation from 'swr/mutation'

import { isDriveEntriesKey, uploadFile } from '@/api/domains/drive'

const MUTATION_KEY = ['drive', 'uploadFile'] as const

export type UploadFileArgument = {
    file: File
    parentId?: string | null
}

export function useApiUploadFile() {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        MUTATION_KEY,
        (_key, { arg }: { arg: UploadFileArgument }) =>
            uploadFile(arg.file, arg.parentId),
        { onSuccess: () => void mutate(isDriveEntriesKey) },
    )
}
