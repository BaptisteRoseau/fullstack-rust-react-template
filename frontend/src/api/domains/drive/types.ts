/** The levels a grant may carry, and the source of {@link PermissionLevel}. */
export const PERMISSION_LEVELS = ['viewer', 'editor', 'manager'] as const

export type PermissionLevel = (typeof PERMISSION_LEVELS)[number]

/** A directory in the caller's tree. `null` parent means the tree's root. */
export type DriveDirectory = {
    id: string
    name: string
    owner: string
    parentId: string | null
    createdAt: Date
    updatedAt: Date
}

/**
 * A stored file. `sizeBytes` is what the user handed over, `storedSizeBytes`
 * what the object store actually holds once compressed and encrypted.
 */
export type DriveFile = {
    id: string
    name: string
    owner: string
    parentId: string | null
    mimeType: string
    sizeBytes: number
    storedSizeBytes: number
    hasThumbnail: boolean
    createdAt: Date
    updatedAt: Date
}

/** One level of the tree: the listed directory, and its direct children. */
export type DriveEntries = {
    directory: DriveDirectory | null
    directories: DriveDirectory[]
    files: DriveFile[]
}

/** One grant on a file or a directory. */
export type PermissionGrant = {
    id: string
    grantee: string
    grantedBy: string
    level: PermissionLevel
    createdAt: Date
    updatedAt: Date
}

export type NewDirectory = {
    name: string
    parentId?: string | null
}

/**
 * A rename, a move, or both. An omitted field is left untouched; a `null`
 * `parentId` moves the entry back to the caller's root.
 */
export type DriveEntryUpdate = {
    name?: string
    parentId?: string | null
}

export type NewPermissionGrant = {
    userId: string
    level: PermissionLevel
}
