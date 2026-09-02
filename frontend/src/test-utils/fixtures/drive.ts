import { randFileName, randUuid, randWord } from '@ngneat/falso'

import type {
    DriveDirectory,
    DriveEntries,
    DriveFile,
    PermissionGrant,
} from '@/api/domains/drive'
import type {
    GetDirectoryResponse,
    GetEntriesResponse,
    GetFileResponse,
    GetPermissionResponse,
} from '@/api/generated'

export function buildDriveDirectory(
    overrides: Partial<DriveDirectory> = {},
): DriveDirectory {
    return {
        id: randUuid(),
        name: randWord(),
        owner: randUuid(),
        parentId: null,
        createdAt: new Date(),
        updatedAt: new Date(),
        ...overrides,
    }
}

export function buildDriveFile(overrides: Partial<DriveFile> = {}): DriveFile {
    return {
        id: randUuid(),
        name: randFileName(),
        owner: randUuid(),
        parentId: null,
        mimeType: 'text/plain',
        sizeBytes: 2048,
        storedSizeBytes: 1024,
        hasThumbnail: false,
        createdAt: new Date(),
        updatedAt: new Date(),
        ...overrides,
    }
}

export function buildPermissionGrant(
    overrides: Partial<PermissionGrant> = {},
): PermissionGrant {
    return {
        id: randUuid(),
        grantee: randUuid(),
        grantedBy: randUuid(),
        level: 'viewer',
        createdAt: new Date(),
        updatedAt: new Date(),
        ...overrides,
    }
}

export function buildDriveEntries(
    overrides: Partial<DriveEntries> = {},
): DriveEntries {
    return { directory: null, directories: [], files: [], ...overrides }
}

/**
 * The wire shapes, which are not the domain shapes: timestamps are RFC 3339
 * strings here and `Date`s above, and an absent parent may be omitted rather
 * than `null`.
 */
export function buildGetDirectoryResponse(
    overrides: Partial<GetDirectoryResponse> = {},
): GetDirectoryResponse {
    return {
        id: randUuid(),
        name: randWord(),
        owner: randUuid(),
        parentId: null,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        ...overrides,
    }
}

export function buildGetFileResponse(
    overrides: Partial<GetFileResponse> = {},
): GetFileResponse {
    return {
        id: randUuid(),
        name: randFileName(),
        owner: randUuid(),
        parentId: null,
        mimeType: 'text/plain',
        sizeBytes: 2048,
        storedSizeBytes: 1024,
        hasThumbnail: false,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        ...overrides,
    }
}

export function buildGetEntriesResponse(
    overrides: Partial<GetEntriesResponse> = {},
): GetEntriesResponse {
    return { directories: [], files: [], directory: null, ...overrides }
}

export function buildGetPermissionResponse(
    overrides: Partial<GetPermissionResponse> = {},
): GetPermissionResponse {
    return {
        id: randUuid(),
        grantee: randUuid(),
        grantedBy: randUuid(),
        level: 'viewer',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        ...overrides,
    }
}
