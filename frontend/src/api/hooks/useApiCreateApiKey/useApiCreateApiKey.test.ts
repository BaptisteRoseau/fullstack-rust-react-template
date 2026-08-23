import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiApiKeys } from '@/api/hooks/useApiApiKeys'
import {
    buildCreateApiKeyResponse,
    buildGetApiKeyResponse,
} from '@/test-utils/fixtures/apiKeys'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiCreateApiKey } from './useApiCreateApiKey'

it('returns the created key with its secret', async () => {
    server.use(
        http.post(endpoint(API_PATHS.apiKeys), () =>
            HttpResponse.json(
                buildCreateApiKeyResponse({ key: 'sk_live_1234' }),
                { status: 201 },
            ),
        ),
    )

    const { result } = renderHook(() => useApiCreateApiKey(), {
        wrapper: SwrWrapper,
    })

    const created = await act(() =>
        result.current.trigger({
            name: 'CI deploy key',
            permissions: ['read'],
        }),
    )

    expect(created.secret, `expected the raw key, got ${created.secret}`).toBe(
        'sk_live_1234',
    )
})

it('refreshes the key list without the call site asking', async () => {
    const names = ['first key', 'second key']
    let listRequests = 0

    server.use(
        http.get(endpoint(API_PATHS.apiKeys), () => {
            const name = names[Math.min(listRequests, names.length - 1)]
            listRequests += 1
            return HttpResponse.json([buildGetApiKeyResponse({ name })])
        }),
        http.post(endpoint(API_PATHS.apiKeys), () =>
            HttpResponse.json(buildCreateApiKeyResponse(), { status: 201 }),
        ),
    )

    const { result } = renderHook(
        () => ({ list: useApiApiKeys(), create: useApiCreateApiKey() }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.[0].name,
            'the list should have loaded first',
        ).toBe('first key'),
    )

    await act(() =>
        result.current.create.trigger({
            name: 'second key',
            permissions: ['read'],
        }),
    )

    await waitFor(() =>
        expect(
            result.current.list.data?.[0].name,
            `the mutation must invalidate the list itself, got ${result.current.list.data?.[0].name}`,
        ).toBe('second key'),
    )
})
