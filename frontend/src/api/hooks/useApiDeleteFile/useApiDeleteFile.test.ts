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

import { useApiDeleteFile } from './useApiDeleteFile'

it('deletes the file and refreshes the listing itself', async () => {
    let deleted = false

    server.use(
        http.get(endpoint(API_PATHS.files), () =>
            HttpResponse.json(
                buildGetEntriesResponse({
                    files: deleted
                        ? []
                        : [buildGetFileResponse({ id: 'file-42' })],
                }),
            ),
        ),
        http.delete(endpoint(`${API_PATHS.files}/:id`), ({ params }) => {
            deleted = String(params.id) === 'file-42'
            return HttpResponse.text(null, { status: 204 })
        }),
    )

    const { result } = renderHook(
        () => ({ list: useApiEntries(), remove: useApiDeleteFile('file-42') }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.files.length,
            'the listing should have loaded first',
        ).toBe(1),
    )

    await act(() => result.current.remove.trigger())

    expect(deleted, 'the delete must target the file the hook was given').toBe(
        true,
    )
    await waitFor(() =>
        expect(
            result.current.list.data?.files.length,
            `the mutation must invalidate the listing itself, got ${result.current.list.data?.files.length}`,
        ).toBe(0),
    )
})
