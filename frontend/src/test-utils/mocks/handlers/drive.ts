import { http, HttpResponse } from 'msw'

import type {
    GetDirectoryResponse,
    GetEntriesResponse,
    GetFileResponse,
    GetPermissionResponse,
    PatchEntryRequest,
    PostDirectoryRequest,
    PutPermissionRequest,
} from '@/api/generated'

import type {
    DirectoryPermissionRecord,
    DirectoryRecord,
    FilePermissionRecord,
    FileRecord,
} from '../db'
import { CURRENT_USER_ID, db, persistDb } from '../db'
import { API_PATHS, endpoint, isAuthenticated, networkDelay } from '../utils'

const UNAUTHORIZED = { id: 'UNAUTHORIZED', error: 'Not authenticated' }

const notFound = (what: string) =>
    HttpResponse.json(
        { id: 'NOT_FOUND', error: `${what} not found` },
        {
            status: 404,
        },
    )

const badRequest = (error: string) =>
    HttpResponse.json({ id: 'BAD_REQUEST', error }, { status: 400 })

/** A real 1×1 WebP, so a preview in dev or Storybook renders an actual image. */
const THUMBNAIL_WEBP =
    'UklGRlYAAABXRUJQVlA4WAoAAAAQAAAAAAAAAAAAQUxQSAwAAAARBxAR/Q9ERP8DAABWUDggGAAAABQBAJ0BKgEAAQAAAP4AAA3AAP7mtQAAAA=='

function toBytes(base64: string): Uint8Array {
    const binary = atob(base64)
    return Uint8Array.from(binary, (character) => character.charCodeAt(0))
}

function toGetDirectoryResponse(
    directory: DirectoryRecord,
): GetDirectoryResponse {
    return {
        id: directory.id,
        name: directory.name,
        owner: directory.owner,
        parentId: directory.parentId,
        createdAt: directory.createdAt,
        updatedAt: directory.updatedAt,
    }
}

/** Built field by field so `content`, which the record carries, cannot leak. */
function toGetFileResponse(file: FileRecord): GetFileResponse {
    return {
        id: file.id,
        name: file.name,
        owner: file.owner,
        parentId: file.parentId,
        mimeType: file.mimeType,
        sizeBytes: file.sizeBytes,
        storedSizeBytes: file.storedSizeBytes,
        hasThumbnail: file.hasThumbnail,
        createdAt: file.createdAt,
        updatedAt: file.updatedAt,
    }
}

function toGetPermissionResponse(
    grant: DirectoryPermissionRecord | FilePermissionRecord,
): GetPermissionResponse {
    return {
        id: grant.id,
        grantee: grant.grantee,
        grantedBy: grant.grantedBy,
        level: grant.level,
        createdAt: grant.createdAt,
        updatedAt: grant.updatedAt,
    }
}

function childrenOf(parentId: string | null) {
    return {
        directories: db.directory
            .findMany((query) =>
                query.where({ parentId: (value) => value === parentId }),
            )
            .map(toGetDirectoryResponse),
        files: db.file
            .findMany((query) =>
                query.where({ parentId: (value) => value === parentId }),
            )
            .map(toGetFileResponse),
    }
}

function applyPatch(
    record: DirectoryRecord | FileRecord,
    body: PatchEntryRequest,
) {
    if (body.name !== undefined && body.name !== null) {
        record.name = body.name
    }
    if (body.parentId !== undefined) {
        record.parentId = body.parentId
    }
    record.updatedAt = new Date().toISOString()
}

const directoryHandlers = [
    http.post(endpoint(API_PATHS.directories), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        const body = (await request.json()) as PostDirectoryRequest
        if (body.name.trim() === '' || body.name.includes('/')) {
            return badRequest('Invalid directory name')
        }
        const now = new Date().toISOString()
        const directory = await db.directory.create({
            name: body.name,
            owner: CURRENT_USER_ID,
            parentId: body.parentId ?? null,
            createdAt: now,
            updatedAt: now,
        })
        await persistDb('directory')
        return HttpResponse.json<GetDirectoryResponse>(
            toGetDirectoryResponse(directory),
            { status: 201 },
        )
    }),

    http.get(
        endpoint(`${API_PATHS.directories}/:id/permissions`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const directoryId = String(params.id)
            if (
                !db.directory.findFirst((query) =>
                    query.where({ id: directoryId }),
                )
            ) {
                return notFound('Directory')
            }
            return HttpResponse.json<GetPermissionResponse[]>(
                db.directoryPermission
                    .findMany((query) => query.where({ directoryId }))
                    .map(toGetPermissionResponse),
            )
        },
    ),

    http.put(
        endpoint(`${API_PATHS.directories}/:id/permissions/:userId`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const directoryId = String(params.id)
            const grantee = String(params.userId)
            if (
                !db.directory.findFirst((query) =>
                    query.where({ id: directoryId }),
                )
            ) {
                return notFound('Directory')
            }
            if (grantee === CURRENT_USER_ID) {
                return badRequest('A user cannot grant to itself')
            }
            const { level } = (await request.json()) as PutPermissionRequest
            const now = new Date().toISOString()
            const existing = db.directoryPermission.findFirst((query) =>
                query.where({ directoryId, grantee }),
            )
            const grant = existing
                ? await db.directoryPermission.update(existing, {
                      data(record) {
                          record.level = level
                          record.updatedAt = now
                      },
                      strict: true,
                  })
                : await db.directoryPermission.create({
                      directoryId,
                      grantee,
                      grantedBy: CURRENT_USER_ID,
                      level,
                      createdAt: now,
                      updatedAt: now,
                  })
            await persistDb('directoryPermission')
            return HttpResponse.json<GetPermissionResponse>(
                toGetPermissionResponse(grant),
            )
        },
    ),

    http.delete(
        endpoint(`${API_PATHS.directories}/:id/permissions/:userId`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const deleted = db.directoryPermission.delete((query) =>
                query.where({
                    directoryId: String(params.id),
                    grantee: String(params.userId),
                }),
            )
            if (!deleted) {
                return notFound('Grant')
            }
            await persistDb('directoryPermission')
            return HttpResponse.text(null, { status: 204 })
        },
    ),

    http.patch(
        endpoint(`${API_PATHS.directories}/:id`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const id = String(params.id)
            const directory = db.directory.findFirst((query) =>
                query.where({ id }),
            )
            if (!directory) {
                return notFound('Directory')
            }
            const body = (await request.json()) as PatchEntryRequest
            if (body.parentId === id) {
                return badRequest('A directory cannot be moved inside itself')
            }
            const updated = await db.directory.update(directory, {
                data: (record) => applyPatch(record, body),
                strict: true,
            })
            await persistDb('directory')
            return HttpResponse.json<GetDirectoryResponse>(
                toGetDirectoryResponse(updated),
            )
        },
    ),

    http.delete(
        endpoint(`${API_PATHS.directories}/:id`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const id = String(params.id)
            const deleted = db.directory.delete((query) => query.where({ id }))
            if (!deleted) {
                return notFound('Directory')
            }
            db.file.deleteMany((query) => query.where({ parentId: id }))
            db.directory.deleteMany((query) => query.where({ parentId: id }))
            await persistDb('directory')
            await persistDb('file')
            return HttpResponse.text(null, { status: 204 })
        },
    ),
]

/**
 * Reading a request body that carries a `Blob` never settles under jsdom, so
 * this resolver cannot run in a Vitest suite — it backs the browser worker and
 * the Express mock server, where `formData()` behaves. A unit test that needs
 * an upload declares its own `server.use` resolver and answers from the
 * headers alone.
 */
const uploadHandlers = [
    http.post(endpoint(API_PATHS.upload), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        const parentId =
            new URL(request.url).searchParams.get('parentId') ?? null
        const uploaded = (await request.formData()).get('file')
        if (!(uploaded instanceof File)) {
            return badRequest('No file field in the request')
        }
        const bytes = new Uint8Array(await uploaded.arrayBuffer())
        const now = new Date().toISOString()
        const file = await db.file.create({
            name: uploaded.name,
            owner: CURRENT_USER_ID,
            parentId,
            mimeType: uploaded.type || 'application/octet-stream',
            sizeBytes: bytes.byteLength,
            storedSizeBytes: Math.max(1, Math.ceil(bytes.byteLength * 0.6)),
            hasThumbnail: uploaded.type.startsWith('image/'),
            content: btoa(String.fromCharCode(...bytes)),
            createdAt: now,
            updatedAt: now,
        })
        await persistDb('file')
        return HttpResponse.json<GetFileResponse>(toGetFileResponse(file), {
            status: 201,
        })
    }),
]

const fileHandlers = [
    http.get(
        endpoint(`${API_PATHS.files}/:id/download`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const file = db.file.findFirst((query) =>
                query.where({ id: String(params.id) }),
            )
            if (!file) {
                return notFound('File')
            }
            return HttpResponse.arrayBuffer(toBytes(file.content).buffer, {
                headers: {
                    'Content-Type': file.mimeType,
                    'Content-Disposition': `attachment; filename="${file.name}"`,
                },
            })
        },
    ),

    http.get(
        endpoint(`${API_PATHS.files}/:id/thumbnail`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const file = db.file.findFirst((query) =>
                query.where({ id: String(params.id) }),
            )
            if (!file?.hasThumbnail) {
                return notFound('Thumbnail')
            }
            return HttpResponse.arrayBuffer(toBytes(THUMBNAIL_WEBP).buffer, {
                headers: { 'Content-Type': 'image/webp' },
            })
        },
    ),

    http.get(
        endpoint(`${API_PATHS.files}/:id/permissions`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const fileId = String(params.id)
            if (!db.file.findFirst((query) => query.where({ id: fileId }))) {
                return notFound('File')
            }
            return HttpResponse.json<GetPermissionResponse[]>(
                db.filePermission
                    .findMany((query) => query.where({ fileId }))
                    .map(toGetPermissionResponse),
            )
        },
    ),

    http.put(
        endpoint(`${API_PATHS.files}/:id/permissions/:userId`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const fileId = String(params.id)
            const grantee = String(params.userId)
            if (!db.file.findFirst((query) => query.where({ id: fileId }))) {
                return notFound('File')
            }
            if (grantee === CURRENT_USER_ID) {
                return badRequest('A user cannot grant to itself')
            }
            const { level } = (await request.json()) as PutPermissionRequest
            const now = new Date().toISOString()
            const existing = db.filePermission.findFirst((query) =>
                query.where({ fileId, grantee }),
            )
            const grant = existing
                ? await db.filePermission.update(existing, {
                      data(record) {
                          record.level = level
                          record.updatedAt = now
                      },
                      strict: true,
                  })
                : await db.filePermission.create({
                      fileId,
                      grantee,
                      grantedBy: CURRENT_USER_ID,
                      level,
                      createdAt: now,
                      updatedAt: now,
                  })
            await persistDb('filePermission')
            return HttpResponse.json<GetPermissionResponse>(
                toGetPermissionResponse(grant),
            )
        },
    ),

    http.delete(
        endpoint(`${API_PATHS.files}/:id/permissions/:userId`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const deleted = db.filePermission.delete((query) =>
                query.where({
                    fileId: String(params.id),
                    grantee: String(params.userId),
                }),
            )
            if (!deleted) {
                return notFound('Grant')
            }
            await persistDb('filePermission')
            return HttpResponse.text(null, { status: 204 })
        },
    ),

    http.get(
        endpoint(`${API_PATHS.files}/:id`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const file = db.file.findFirst((query) =>
                query.where({ id: String(params.id) }),
            )
            return file
                ? HttpResponse.json<GetFileResponse>(toGetFileResponse(file))
                : notFound('File')
        },
    ),

    http.patch(
        endpoint(`${API_PATHS.files}/:id`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const file = db.file.findFirst((query) =>
                query.where({ id: String(params.id) }),
            )
            if (!file) {
                return notFound('File')
            }
            const body = (await request.json()) as PatchEntryRequest
            const updated = await db.file.update(file, {
                data: (record) => applyPatch(record, body),
                strict: true,
            })
            await persistDb('file')
            return HttpResponse.json<GetFileResponse>(
                toGetFileResponse(updated),
            )
        },
    ),

    http.delete(
        endpoint(`${API_PATHS.files}/:id`),
        async ({ params, request }) => {
            await networkDelay()
            if (!isAuthenticated(request)) {
                return HttpResponse.json(UNAUTHORIZED, { status: 401 })
            }
            const deleted = db.file.delete((query) =>
                query.where({ id: String(params.id) }),
            )
            if (!deleted) {
                return notFound('File')
            }
            await persistDb('file')
            return HttpResponse.text(null, { status: 204 })
        },
    ),
]

/**
 * The listing comes last: `/files/:id` would otherwise swallow
 * `/files/directories` and `/files/upload`, which MSW matches in order.
 */
const listHandlers = [
    http.get(endpoint(API_PATHS.files), async ({ request }) => {
        await networkDelay()
        if (!isAuthenticated(request)) {
            return HttpResponse.json(UNAUTHORIZED, { status: 401 })
        }
        const parentId = new URL(request.url).searchParams.get('parentId')
        if (!parentId) {
            return HttpResponse.json<GetEntriesResponse>({
                directory: null,
                ...childrenOf(null),
            })
        }
        const directory = db.directory.findFirst((query) =>
            query.where({ id: parentId }),
        )
        if (!directory) {
            return notFound('Directory')
        }
        return HttpResponse.json<GetEntriesResponse>({
            directory: toGetDirectoryResponse(directory),
            ...childrenOf(parentId),
        })
    }),
]

export const driveHandlers = [
    ...directoryHandlers,
    ...uploadHandlers,
    ...fileHandlers,
    ...listHandlers,
]
