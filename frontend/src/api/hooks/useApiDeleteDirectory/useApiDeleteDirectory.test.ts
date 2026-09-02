import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiEntries } from '@/api/hooks/useApiEntries'
import {
    buildGetDirectoryResponse,
    buildGetEntriesResponse,
} from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiDeleteDirectory } from './useApiDeleteDirectory'

it('deletes the directory and refreshes the listing itself', async () => {
    let deleted = false

    server.use(
        http.get(endpoint(API_PATHS.files), () =>
            HttpResponse.json(
                buildGetEntriesResponse({
                    directories: deleted
                        ? []
                        : [buildGetDirectoryResponse({ id: 'dir-42' })],
                }),
            ),
        ),
        http.delete(endpoint(`${API_PATHS.directories}/:id`), ({ params }) => {
            deleted = String(params.id) === 'dir-42'
            return HttpResponse.text(null, { status: 204 })
        }),
    )

    const { result } = renderHook(
        () => ({
            list: useApiEntries(),
            remove: useApiDeleteDirectory('dir-42'),
        }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.directories.length,
            'the listing should have loaded first',
        ).toBe(1),
    )

    await act(() => result.current.remove.trigger())

    expect(
        deleted,
        'the delete must target the directory the hook was given',
    ).toBe(true)
    await waitFor(() =>
        expect(
            result.current.list.data?.directories.length,
            `the mutation must invalidate the listing itself, got ${result.current.list.data?.directories.length}`,
        ).toBe(0),
    )
})
