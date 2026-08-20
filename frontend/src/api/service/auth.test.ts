import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { ME_ENDPOINT } from '@/api/auth'
import { buildCurrentUser } from '@/test-utils/fixtures/auth'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useCurrentUser } from './auth'

it('returns the signed-in user', async () => {
    const user = buildCurrentUser({ email: 'ada@example.com' })
    server.use(http.get(`*${ME_ENDPOINT}`, () => HttpResponse.json(user)))

    const { result } = renderHook(() => useCurrentUser(), {
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
        http.get(`*${ME_ENDPOINT}`, () =>
            HttpResponse.json({ error: 'Not authenticated' }, { status: 401 }),
        ),
    )

    const { result } = renderHook(() => useCurrentUser(), {
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
