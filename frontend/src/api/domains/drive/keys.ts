/**
 * The listing key carries the directory it lists: every level of the tree
 * caches on its own, and `'root'` stands in for the absent parent so the key
 * never holds `undefined`.
 */
export const driveKeys = {
    all: ['drive'] as const,
    entries: (parentId?: string | null) =>
        ['drive', 'entries', parentId ?? 'root'] as const,
    file: (fileId: string) => ['drive', 'file', fileId] as const,
    content: (fileId: string) => ['drive', 'content', fileId] as const,
    thumbnail: (fileId: string) => ['drive', 'thumbnail', fileId] as const,
    directoryPermissions: (directoryId: string) =>
        ['drive', 'directoryPermissions', directoryId] as const,
    filePermissions: (fileId: string) =>
        ['drive', 'filePermissions', fileId] as const,
}

/**
 * Every listing at once. A create, a move or a delete changes the level it
 * left and the level it landed in, and only one of the two is ever the one on
 * screen, so a mutation invalidates the lot rather than guessing.
 */
export const isDriveEntriesKey = (key: unknown): boolean =>
    Array.isArray(key) && key[0] === 'drive' && key[1] === 'entries'
