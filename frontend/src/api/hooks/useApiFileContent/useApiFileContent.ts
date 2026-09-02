import useSWR from 'swr'

import { downloadFileContent, driveKeys } from '@/api/domains/drive'

/**
 * The file's own bytes, for a preview. Kept out of {@link useApiDownloadFile},
 * which saves to disk rather than rendering, because only this one belongs in
 * the cache: a preview is opened again and again, a save is a one-off.
 */
export function useApiFileContent(fileId: string | undefined) {
    return useSWR(fileId ? driveKeys.content(fileId) : null, ([, , id]) =>
        downloadFileContent(id),
    )
}
