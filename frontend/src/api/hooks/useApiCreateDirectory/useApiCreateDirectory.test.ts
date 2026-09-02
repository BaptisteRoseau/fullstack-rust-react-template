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

import { useApiCreateDirectory } from './useApiCreateDirectory'

it('returns the created directory', async () => {
    server.use(
        http.post(endpoint(API_PATHS.directories), () =>
            HttpResponse.json(buildGetDirectoryResponse({ name: 'Invoices' }), {
                status: 201,
            }),
        ),
    )

    const { result } = renderHook(() => useApiCreateDirectory(), {
        wrapper: SwrWrapper,
    })

    const created = await act(() =>
        result.current.trigger({ name: 'Invoices' }),
    )

    expect(created.name, `expected "Invoices", got "${created.name}"`).toBe(
        'Invoices',
    )
})

it('refreshes the listing without the call site asking', async () => {
    const names = ['Invoices', 'Contracts']
    let listRequests = 0

    server.use(
        http.get(endpoint(API_PATHS.files), () => {
            const name = names[Math.min(listRequests, names.length - 1)]
            listRequests += 1
            return HttpResponse.json(
                buildGetEntriesResponse({
                    directories: [buildGetDirectoryResponse({ name })],
                }),
            )
        }),
        http.post(endpoint(API_PATHS.directories), () =>
            HttpResponse.json(buildGetDirectoryResponse(), { status: 201 }),
        ),
    )

    const { result } = renderHook(
        () => ({ list: useApiEntries(), create: useApiCreateDirectory() }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.directories[0].name,
            'the listing should have loaded first',
        ).toBe('Invoices'),
    )

    await act(() => result.current.create.trigger({ name: 'Contracts' }))

    await waitFor(() =>
        expect(
            result.current.list.data?.directories[0].name,
            `the mutation must invalidate the listing itself, got ${result.current.list.data?.directories[0].name}`,
        ).toBe('Contracts'),
    )
})
