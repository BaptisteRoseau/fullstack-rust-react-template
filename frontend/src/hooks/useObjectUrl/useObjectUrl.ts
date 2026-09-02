import { useEffect, useMemo } from 'react'

/**
 * Exposes a `blob:` URL for as long as the blob it was made from is the current
 * one. The URL is revoked whenever the blob changes and when the component
 * unmounts: an object URL pins its blob in memory until it is, and a file
 * browser makes one per thumbnail.
 *
 * The URL is derived during render rather than pushed into state from an
 * effect, so no render ever hands out a URL pointing at the previous blob.
 */
export function useObjectUrl(blob: Blob | undefined | null): string | null {
    const url = useMemo(() => (blob ? URL.createObjectURL(blob) : null), [blob])

    useEffect(() => {
        if (!url) {
            return
        }

        return () => {
            URL.revokeObjectURL(url)
        }
    }, [url])

    return url
}
