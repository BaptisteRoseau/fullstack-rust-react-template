import useSWR from 'swr'

import { downloadFileThumbnail, driveKeys } from '@/api/domains/drive'
import { useObjectUrl } from '@/hooks/useObjectUrl'

/**
 * A thumbnail is binary, so it is fetched once through SWR — which dedupes the
 * request across every card showing the same file — and handed to the DOM as an
 * object URL that {@link useObjectUrl} revokes on its own.
 */
export function useApiThumbnail(fileId: string | undefined) {
    const { data, error, isLoading } = useSWR(
        fileId ? driveKeys.thumbnail(fileId) : null,
        ([, , id]) => downloadFileThumbnail(id),
        { shouldRetryOnError: false },
    )

    return { url: useObjectUrl(data), error, isLoading }
}
