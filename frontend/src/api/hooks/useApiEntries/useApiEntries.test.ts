import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import {
    buildGetDirectoryResponse,
    buildGetEntriesResponse,
} from '@/test-utils/fixtures/drive'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiEntries } from './useApiEntries'

it('lists the root when no directory is named', async () => {
    let requestedParentId: string | null = 'unset'

    server.use(
        http.get(endpoint(API_PATHS.files), ({ request }) => {
            requestedParentId = new URL(request.url).searchParams.get(
                'parentId',
            )
            return HttpResponse.json(
                buildGetEntriesResponse({
                    directories: [
                        buildGetDirectoryResponse({ name: 'Invoices' }),
                    ],
                }),
            )
        }),
    )

    const { result } = renderHook(() => useApiEntries(), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.directories[0].name,
            `expected "Invoices", got ${result.current.data?.directories[0].name} (error: ${result.current.error})`,
        ).toBe('Invoices'),
    )
    expect(
        requestedParentId,
        `the root listing must send no parent, got ${String(requestedParentId)}`,
    ).toBeNull()
})

it('caches each directory on its own key', async () => {
    const requested: (string | null)[] = []

    server.use(
        http.get(endpoint(API_PATHS.files), ({ request }) => {
            requested.push(new URL(request.url).searchParams.get('parentId'))
            return HttpResponse.json(buildGetEntriesResponse())
        }),
    )

    const { result } = renderHook(
        () => ({ root: useApiEntries(), child: useApiEntries('dir-1') }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            requested.length,
            `expected 2 requests, got ${requested.length}`,
        ).toBe(2),
    )
    expect(
        result.current.child.error,
        `expected no error, got ${result.current.child.error}`,
    ).toBeUndefined()
    expect(
        requested.sort(),
        `expected one root and one child request, got ${JSON.stringify(requested)}`,
    ).toEqual(['dir-1', null])
})
