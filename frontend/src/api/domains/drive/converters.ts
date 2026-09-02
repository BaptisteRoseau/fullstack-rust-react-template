import type {
    GetDirectoryResponse,
    GetEntriesResponse,
    GetFileResponse,
    GetPermissionResponse,
    PatchEntryRequest,
    PostDirectoryRequest,
    PutPermissionRequest,
} from '@/api/generated'

import {
    PERMISSION_LEVELS,
    type DriveDirectory,
    type DriveEntries,
    type DriveEntryUpdate,
    type DriveFile,
    type NewDirectory,
    type PermissionGrant,
    type PermissionLevel,
} from './types'

const isPermissionLevel = (value: string): value is PermissionLevel =>
    (PERMISSION_LEVELS as readonly string[]).includes(value)

/**
 * A level the frontend does not know yet reads as the least privileged one:
 * dropping the grant instead would hide a share that really exists.
 */
export function toPermissionLevel(value: string): PermissionLevel {
    return isPermissionLevel(value) ? value : 'viewer'
}

export function fromGetDirectoryResponse(
    response: GetDirectoryResponse,
): DriveDirectory {
    return {
        id: response.id,
        name: response.name,
        owner: response.owner,
        parentId: response.parentId ?? null,
        createdAt: new Date(response.createdAt),
        updatedAt: new Date(response.updatedAt),
    }
}

export function fromGetFileResponse(response: GetFileResponse): DriveFile {
    return {
        id: response.id,
        name: response.name,
        owner: response.owner,
        parentId: response.parentId ?? null,
        mimeType: response.mimeType,
        sizeBytes: response.sizeBytes,
        storedSizeBytes: response.storedSizeBytes,
        hasThumbnail: response.hasThumbnail,
        createdAt: new Date(response.createdAt),
        updatedAt: new Date(response.updatedAt),
    }
}

export function fromGetEntriesResponse(
    response: GetEntriesResponse,
): DriveEntries {
    return {
        directory: response.directory
            ? fromGetDirectoryResponse(response.directory)
            : null,
        directories: response.directories.map(fromGetDirectoryResponse),
        files: response.files.map(fromGetFileResponse),
    }
}

export function fromGetPermissionResponse(
    response: GetPermissionResponse,
): PermissionGrant {
    return {
        id: response.id,
        grantee: response.grantee,
        grantedBy: response.grantedBy,
        level: toPermissionLevel(response.level),
        createdAt: new Date(response.createdAt),
        updatedAt: new Date(response.updatedAt),
    }
}

export function toPostDirectoryRequest(
    directory: NewDirectory,
): PostDirectoryRequest {
    return { name: directory.name, parentId: directory.parentId ?? null }
}

/**
 * Only the fields the caller actually set are sent: the backend leaves an
 * omitted field untouched, and `parentId: null` is a move to the root rather
 * than "no change".
 */
export function toPatchEntryRequest(
    update: DriveEntryUpdate,
): PatchEntryRequest {
    const request: PatchEntryRequest = {}
    if (update.name !== undefined) {
        request.name = update.name
    }
    if (update.parentId !== undefined) {
        request.parentId = update.parentId
    }
    return request
}

export function toPutPermissionRequest(
    level: PermissionLevel,
): PutPermissionRequest {
    return { level }
}
