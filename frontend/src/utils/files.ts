const SIZE_UNITS = ['B', 'KB', 'MB', 'GB', 'TB'] as const

const BYTES_PER_UNIT = 1024

/**
 * A byte count in the largest unit that keeps it readable. Units stay
 * untranslated on purpose: `MB` is what a file manager shows in every locale,
 * and this module is a pure helper with no access to the catalogs.
 */
export function formatFileSize(bytes: number): string {
    if (!Number.isFinite(bytes) || bytes < 0) {
        return `0 ${SIZE_UNITS[0]}`
    }

    let value = bytes
    let unit = 0
    while (value >= BYTES_PER_UNIT && unit < SIZE_UNITS.length - 1) {
        value /= BYTES_PER_UNIT
        unit += 1
    }

    const decimals = unit === 0 || value >= 100 ? 0 : 1
    return `${value.toFixed(decimals)} ${SIZE_UNITS[unit]}`
}

/**
 * How much of a file the server's compression and encryption saved, rounded to
 * a whole percent. `null` when there is nothing to boast about — an empty file,
 * or content that came out no smaller than it went in.
 */
export function savedPercentage(
    sizeBytes: number,
    storedSizeBytes: number,
): number | null {
    if (sizeBytes <= 0 || storedSizeBytes >= sizeBytes) {
        return null
    }
    return Math.round(((sizeBytes - storedSizeBytes) / sizeBytes) * 100)
}

/** The part of a MIME type before the slash: `image`, `text`, `application`. */
export function mimeTypeGroup(mimeType: string): string {
    return mimeType.split('/')[0] ?? ''
}

export function isPdf(mimeType: string): boolean {
    return mimeType.startsWith('application/pdf')
}
