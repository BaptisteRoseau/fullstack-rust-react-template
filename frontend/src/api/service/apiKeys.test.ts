import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { API_KEYS_ENDPOINT } from '@/api/apiKeys'
import { buildApiKey } from '@/test-utils/fixtures/apiKeys'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiKeys } from './apiKeys'

it('returns the api keys of the caller', async () => {
    const apiKey = buildApiKey({ name: 'CI deploy key' })
    server.use(
        http.get(`*${API_KEYS_ENDPOINT}`, () => HttpResponse.json([apiKey])),
    )

    const { result } = renderHook(() => useApiKeys(), { wrapper: SwrWrapper })

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
