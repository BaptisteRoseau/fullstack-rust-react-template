import { useCallback, useState } from 'react'

import { downloadFileContent } from '@/api/domains/drive'

/**
 * Saving a file is a one-off command, not a cache entry, so it is a callback
 * rather than an SWR read. The temporary anchor is the only way a fetched blob
 * can reach the user's disk with the name the drive gave it; its object URL is
 * revoked as soon as the click has been dispatched.
 */
function saveBlob(blob: Blob, fileName: string) {
    const url = URL.createObjectURL(blob)
    const anchor = document.createElement('a')
    anchor.href = url
    anchor.download = fileName
    document.body.append(anchor)
    anchor.click()
    anchor.remove()
    URL.revokeObjectURL(url)
}

export function useApiDownloadFile() {
    const [isDownloading, setIsDownloading] = useState(false)

    const download = useCallback(async (fileId: string, fileName: string) => {
        setIsDownloading(true)
        try {
            saveBlob(await downloadFileContent(fileId), fileName)
        } finally {
            setIsDownloading(false)
        }
    }, [])

    return { download, isDownloading }
}
