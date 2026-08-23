import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetApiKeyResponse } from '@/test-utils/fixtures/apiKeys'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiApiKeys } from './useApiApiKeys'

it('returns the caller keys', async () => {
    server.use(
        http.get(endpoint(API_PATHS.apiKeys), () =>
            HttpResponse.json([
                buildGetApiKeyResponse({ name: 'CI deploy key' }),
            ]),
        ),
    )

    const { result } = renderHook(() => useApiApiKeys(), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.length,
            `expected 1 key, got ${result.current.data?.length} (error: ${result.current.error})`,
        ).toBe(1),
    )
    expect(
        result.current.data?.[0].name,
        `expected "CI deploy key", got "${result.current.data?.[0].name}"`,
    ).toBe('CI deploy key')
})
