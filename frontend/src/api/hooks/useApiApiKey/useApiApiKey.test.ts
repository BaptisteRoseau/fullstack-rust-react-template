import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetApiKeyResponse } from '@/test-utils/fixtures/apiKeys'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiApiKey } from './useApiApiKey'

it('requests the key named by its cache key', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.apiKeys}/:id`), ({ params }) =>
            HttpResponse.json(
                buildGetApiKeyResponse({ id: String(params.id) }),
            ),
        ),
    )

    const { result } = renderHook(() => useApiApiKey('key-42'), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.id,
            `expected key-42, got ${result.current.data?.id} (error: ${result.current.error})`,
        ).toBe('key-42'),
    )
})

it('skips the request entirely without an id', async () => {
    let requests = 0
    server.use(
        http.get(endpoint(`${API_PATHS.apiKeys}/:id`), () => {
            requests += 1
            return HttpResponse.json(buildGetApiKeyResponse())
        }),
    )

    const { result } = renderHook(() => useApiApiKey(undefined), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(result.current.isLoading, 'the hook should have settled').toBe(
            false,
        ),
    )
    expect(requests, `expected no request, got ${requests}`).toBe(0)
})
