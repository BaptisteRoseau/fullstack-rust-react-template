import { http, HttpResponse } from 'msw'

import {
    buildGetDirectoryResponse,
    buildGetEntriesResponse,
    buildGetFileResponse,
} from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'

import {
    downloadFileContent,
    fetchEntries,
    updateFile,
    uploadFile,
} from './drive'

it('returns a listing as domain objects', async () => {
    server.use(
        http.get(endpoint(API_PATHS.files), () =>
            HttpResponse.json(
                buildGetEntriesResponse({
                    directories: [
                        buildGetDirectoryResponse({ name: 'Invoices' }),
                    ],
                    files: [buildGetFileResponse({ name: 'notes.txt' })],
                }),
            ),
        ),
    )

    const entries = await fetchEntries()

    expect(
        entries.directories[0].createdAt instanceof Date,
        'the fetcher must hand back the converted shape, not the wire one',
    ).toBe(true)
    expect(
        entries.files[0].name,
        `expected "notes.txt", got "${entries.files[0].name}"`,
    ).toBe('notes.txt')
})

it('asks for the directory it was given', async () => {
    let requestedParentId: string | null = null

    server.use(
        http.get(endpoint(API_PATHS.files), ({ request }) => {
            requestedParentId = new URL(request.url).searchParams.get(
                'parentId',
            )
            return HttpResponse.json(buildGetEntriesResponse())
        }),
    )

    await fetchEntries('directory-42')

    expect(
        requestedParentId,
        `expected directory-42, got ${String(requestedParentId)}`,
    ).toBe('directory-42')
})

/**
 * The body is deliberately not read back: under jsdom, awaiting a request body
 * that carries a `Blob` never settles. The content type and the destination are
 * what this fetcher decides, and both are observable from the headers and URL.
 */
it('sends the upload as multipart to the destination directory', async () => {
    let contentType: string | null = null
    let requestedParentId: string | null = null

    server.use(
        http.post(endpoint(API_PATHS.upload), ({ request }) => {
            contentType = request.headers.get('content-type')
            requestedParentId = new URL(request.url).searchParams.get(
                'parentId',
            )
            return HttpResponse.json(
                buildGetFileResponse({ name: 'report.txt' }),
                { status: 201 },
            )
        }),
    )

    const file = await uploadFile(
        new File(['hello'], 'report.txt', { type: 'text/plain' }),
        'directory-42',
    )

    expect(
        contentType,
        `expected a multipart body, got ${String(contentType)}`,
    ).toMatch(/^multipart\/form-data/)
    expect(
        requestedParentId,
        `expected directory-42, got ${String(requestedParentId)}`,
    ).toBe('directory-42')
    expect(file.name, `expected the converted file, got "${file.name}"`).toBe(
        'report.txt',
    )
})

it('sends only the fields a rename touches', async () => {
    let body: unknown = null

    server.use(
        http.patch(endpoint(`${API_PATHS.files}/:id`), async ({ request }) => {
            body = await request.json()
            return HttpResponse.json(
                buildGetFileResponse({ name: 'renamed.txt' }),
            )
        }),
    )

    await updateFile('file-1', { name: 'renamed.txt' })

    expect(
        body,
        `a rename must not imply a move, got ${JSON.stringify(body)}`,
    ).toEqual({ name: 'renamed.txt' })
})

it('hands back a download as binary content', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.files}/:id/download`), () =>
            HttpResponse.arrayBuffer(new Uint8Array([1, 2, 3]).buffer, {
                headers: { 'Content-Type': 'application/octet-stream' },
            }),
        ),
    )

    const blob = await downloadFileContent('file-1')

    expect(
        blob.size,
        `expected the 3 bytes the server sent, got ${blob.size}`,
    ).toBe(3)
})
