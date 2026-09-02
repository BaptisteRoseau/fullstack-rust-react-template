import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiEntries } from '@/api/hooks/useApiEntries'
import {
    buildGetEntriesResponse,
    buildGetFileResponse,
} from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiUploadFile } from './useApiUploadFile'

/**
 * The resolvers never read the request body: awaiting a body that carries a
 * `Blob` never settles under jsdom. What the hook decides — the destination and
 * the refreshed listing — is observable without it.
 */
it('uploads into the directory it is given', async () => {
    let requestedParentId: string | null = null

    server.use(
        http.post(endpoint(API_PATHS.upload), ({ request }) => {
            requestedParentId = new URL(request.url).searchParams.get(
                'parentId',
            )
            return HttpResponse.json(
                buildGetFileResponse({ name: 'report.pdf' }),
                { status: 201 },
            )
        }),
    )

    const { result } = renderHook(() => useApiUploadFile(), {
        wrapper: SwrWrapper,
    })

    const uploaded = await act(() =>
        result.current.trigger({
            file: new File(['hello'], 'report.pdf', {
                type: 'application/pdf',
            }),
            parentId: 'dir-42',
        }),
    )

    expect(
        requestedParentId,
        `expected dir-42, got ${String(requestedParentId)}`,
    ).toBe('dir-42')
    expect(uploaded.name, `expected "report.pdf", got "${uploaded.name}"`).toBe(
        'report.pdf',
    )
})

it('refreshes the listing without the call site asking', async () => {
    let uploaded = false

    server.use(
        http.get(endpoint(API_PATHS.files), () =>
            HttpResponse.json(
                buildGetEntriesResponse({
                    files: uploaded
                        ? [buildGetFileResponse({ name: 'report.pdf' })]
                        : [],
                }),
            ),
        ),
        http.post(endpoint(API_PATHS.upload), () => {
            uploaded = true
            return HttpResponse.json(buildGetFileResponse(), { status: 201 })
        }),
    )

    const { result } = renderHook(
        () => ({ list: useApiEntries(), upload: useApiUploadFile() }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.files.length,
            'the listing should have loaded first',
        ).toBe(0),
    )

    await act(() =>
        result.current.upload.trigger({
            file: new File(['hello'], 'report.pdf'),
        }),
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.files.length,
            `the mutation must invalidate the listing itself, got ${result.current.list.data?.files.length}`,
        ).toBe(1),
    )
})
