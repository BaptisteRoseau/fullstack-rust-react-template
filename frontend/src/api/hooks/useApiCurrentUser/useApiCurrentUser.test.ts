import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { buildGetMeResponse } from '@/test-utils/fixtures/auth'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiCurrentUser } from './useApiCurrentUser'

it('returns the signed-in user', async () => {
    server.use(
        http.get(endpoint(API_PATHS.me), () =>
            HttpResponse.json(buildGetMeResponse({ email: 'ada@example.com' })),
        ),
    )

    const { result } = renderHook(() => useApiCurrentUser(), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.email,
            `expected ada@example.com, got ${result.current.data?.email} (error: ${result.current.error})`,
        ).toBe('ada@example.com'),
    )
})

it('resolves to null when the session is missing', async () => {
    server.use(
        http.get(endpoint(API_PATHS.me), () =>
            HttpResponse.json(
                { error: 'Not authenticated', id: 'UNAUTHORIZED' },
                { status: 401 },
            ),
        ),
        http.post(endpoint(API_PATHS.refresh), () =>
            HttpResponse.json(
                { error: 'Not authenticated', id: 'UNAUTHORIZED' },
                { status: 401 },
            ),
        ),
    )

    const { result } = renderHook(() => useApiCurrentUser(), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(result.current.isLoading, 'the hook should have settled').toBe(
            false,
        ),
    )
    expect(
        result.current.data,
        `expected null, got ${JSON.stringify(result.current.data)}`,
    ).toBeNull()
})
