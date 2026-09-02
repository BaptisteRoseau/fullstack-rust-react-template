import {
    buildGetDirectoryResponse,
    buildGetEntriesResponse,
    buildGetFileResponse,
    buildGetPermissionResponse,
} from '@/test-utils/fixtures/drive'

import {
    fromGetDirectoryResponse,
    fromGetEntriesResponse,
    fromGetFileResponse,
    fromGetPermissionResponse,
    toPatchEntryRequest,
    toPostDirectoryRequest,
} from './converters'

it('turns the wire timestamps into Dates', () => {
    const directory = fromGetDirectoryResponse(
        buildGetDirectoryResponse({ createdAt: '2026-01-15T09:30:00Z' }),
    )

    expect(
        directory.createdAt.toISOString(),
        `expected the wire instant to survive, got ${directory.createdAt.toISOString()}`,
    ).toBe('2026-01-15T09:30:00.000Z')
})

it('normalises an omitted parent to null', () => {
    const response = buildGetFileResponse()
    delete response.parentId

    const file = fromGetFileResponse(response)

    expect(
        file.parentId,
        `expected null for an absent parent, got ${String(file.parentId)}`,
    ).toBeNull()
})

it('reads an unknown permission level as the least privileged one', () => {
    const grant = fromGetPermissionResponse({
        ...buildGetPermissionResponse(),
        level: 'archivist' as never,
    })

    expect(
        grant.level,
        `expected the grant to survive as viewer, got ${grant.level}`,
    ).toBe('viewer')
})

it('reports the root listing as a directory-less level', () => {
    const entries = fromGetEntriesResponse(
        buildGetEntriesResponse({
            directories: [buildGetDirectoryResponse({ name: 'Invoices' })],
            files: [buildGetFileResponse({ name: 'notes.txt' })],
        }),
    )

    expect(
        entries.directory,
        `expected no directory at the root, got ${JSON.stringify(entries.directory)}`,
    ).toBeNull()
    expect(
        entries.directories[0].name,
        `expected "Invoices", got "${entries.directories[0].name}"`,
    ).toBe('Invoices')
    expect(
        entries.files[0].name,
        `expected "notes.txt", got "${entries.files[0].name}"`,
    ).toBe('notes.txt')
})

it('sends the caller root as an explicit null parent', () => {
    const request = toPostDirectoryRequest({ name: 'Invoices' })

    expect(
        request.parentId,
        `expected null, got ${String(request.parentId)}`,
    ).toBeNull()
})

it('omits from a patch the fields the caller left alone', () => {
    const request = toPatchEntryRequest({ name: 'renamed.txt' })

    expect(
        'parentId' in request,
        `a move must not be implied by a rename, got ${JSON.stringify(request)}`,
    ).toBe(false)
    expect(
        toPatchEntryRequest({ parentId: null }).parentId,
        'a null parent is a move to the root and must be sent',
    ).toBeNull()
})
