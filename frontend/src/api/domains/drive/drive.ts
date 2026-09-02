import { apiCall } from '@/api/client'
import { ApiError } from '@/api/errors'
import {
    createDirectory as createDirectoryRequest,
    deleteDirectory as deleteDirectoryRequest,
    deleteFile as deleteFileRequest,
    downloadFile as downloadFileRequest,
    downloadThumbnail as downloadThumbnailRequest,
    getFile,
    grantDirectoryPermission as grantDirectoryPermissionRequest,
    grantFilePermission as grantFilePermissionRequest,
    listDirectoryPermissions,
    listEntries,
    listFilePermissions,
    revokeDirectoryPermission as revokeDirectoryPermissionRequest,
    revokeFilePermission as revokeFilePermissionRequest,
    updateDirectory as updateDirectoryRequest,
    updateFile as updateFileRequest,
    uploadFile as uploadFileRequest,
    type UploadFileData,
} from '@/api/generated'

import {
    fromGetDirectoryResponse,
    fromGetEntriesResponse,
    fromGetFileResponse,
    fromGetPermissionResponse,
    toPatchEntryRequest,
    toPostDirectoryRequest,
    toPutPermissionRequest,
} from './converters'
import type {
    DriveDirectory,
    DriveEntries,
    DriveEntryUpdate,
    DriveFile,
    NewDirectory,
    NewPermissionGrant,
    PermissionGrant,
} from './types'

export async function fetchEntries(
    parentId?: string | null,
): Promise<DriveEntries> {
    return fromGetEntriesResponse(
        await apiCall(() =>
            listEntries(parentId ? { query: { parentId } } : {}),
        ),
    )
}

export async function createDirectory(
    directory: NewDirectory,
): Promise<DriveDirectory> {
    return fromGetDirectoryResponse(
        await apiCall(() =>
            createDirectoryRequest({
                body: toPostDirectoryRequest(directory),
            }),
        ),
    )
}

export async function updateDirectory(
    directoryId: string,
    update: DriveEntryUpdate,
): Promise<DriveDirectory> {
    return fromGetDirectoryResponse(
        await apiCall(() =>
            updateDirectoryRequest({
                path: { id: directoryId },
                body: toPatchEntryRequest(update),
            }),
        ),
    )
}

export async function deleteDirectory(directoryId: string): Promise<void> {
    await apiCall(() => deleteDirectoryRequest({ path: { id: directoryId } }))
}

/**
 * The OpenAPI document describes the multipart body as a plain string, so the
 * generated type cannot name the `file` field the backend reads. The SDK call
 * itself is built with `formDataBodySerializer`, which expects exactly this
 * object, hence the cast at the one place that knows the real shape.
 */
export async function uploadFile(
    file: File,
    parentId?: string | null,
): Promise<DriveFile> {
    return fromGetFileResponse(
        await apiCall(() =>
            uploadFileRequest({
                body: { file } as unknown as UploadFileData['body'],
                query: parentId ? { parentId } : undefined,
            }),
        ),
    )
}

export async function fetchFile(fileId: string): Promise<DriveFile> {
    return fromGetFileResponse(
        await apiCall(() => getFile({ path: { id: fileId } })),
    )
}

export async function updateFile(
    fileId: string,
    update: DriveEntryUpdate,
): Promise<DriveFile> {
    return fromGetFileResponse(
        await apiCall(() =>
            updateFileRequest({
                path: { id: fileId },
                body: toPatchEntryRequest(update),
            }),
        ),
    )
}

export async function deleteFile(fileId: string): Promise<void> {
    await apiCall(() => deleteFileRequest({ path: { id: fileId } }))
}

function asBlob(content: unknown): Blob {
    if (content instanceof Blob) {
        return content
    }
    throw new ApiError(
        'The server did not answer with binary content',
        200,
        'PARSE',
        content,
    )
}

/**
 * `parseAs: 'blob'` is explicit rather than left to the client's content-type
 * sniffing: a download must never be re-parsed as JSON or text, which would
 * corrupt the bytes.
 */
export async function downloadFileContent(fileId: string): Promise<Blob> {
    return asBlob(
        await apiCall(() =>
            downloadFileRequest({
                path: { id: fileId },
                parseAs: 'blob',
            }),
        ),
    )
}

export async function downloadFileThumbnail(fileId: string): Promise<Blob> {
    return asBlob(
        await apiCall(() =>
            downloadThumbnailRequest({
                path: { id: fileId },
                parseAs: 'blob',
            }),
        ),
    )
}

export async function fetchDirectoryPermissions(
    directoryId: string,
): Promise<PermissionGrant[]> {
    const response = await apiCall(() =>
        listDirectoryPermissions({ path: { id: directoryId } }),
    )
    return response.map(fromGetPermissionResponse)
}

export async function grantDirectoryPermission(
    directoryId: string,
    grant: NewPermissionGrant,
): Promise<PermissionGrant> {
    return fromGetPermissionResponse(
        await apiCall(() =>
            grantDirectoryPermissionRequest({
                path: { id: directoryId, userId: grant.userId },
                body: toPutPermissionRequest(grant.level),
            }),
        ),
    )
}

export async function revokeDirectoryPermission(
    directoryId: string,
    userId: string,
): Promise<void> {
    await apiCall(() =>
        revokeDirectoryPermissionRequest({
            path: { id: directoryId, userId },
        }),
    )
}

export async function fetchFilePermissions(
    fileId: string,
): Promise<PermissionGrant[]> {
    const response = await apiCall(() =>
        listFilePermissions({ path: { id: fileId } }),
    )
    return response.map(fromGetPermissionResponse)
}

export async function grantFilePermission(
    fileId: string,
    grant: NewPermissionGrant,
): Promise<PermissionGrant> {
    return fromGetPermissionResponse(
        await apiCall(() =>
            grantFilePermissionRequest({
                path: { id: fileId, userId: grant.userId },
                body: toPutPermissionRequest(grant.level),
            }),
        ),
    )
}

export async function revokeFilePermission(
    fileId: string,
    userId: string,
): Promise<void> {
    await apiCall(() =>
        revokeFilePermissionRequest({ path: { id: fileId, userId } }),
    )
}
