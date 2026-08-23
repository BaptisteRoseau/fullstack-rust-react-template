import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiUser } from './useApiUser'

it('returns the user named by its cache key', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.users}/:uuid`), ({ params }) =>
            HttpResponse.json({ name: `user ${String(params.uuid)}` }),
        ),
    )

    const { result } = renderHook(() => useApiUser('user-7'), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(
            result.current.data?.name,
            `expected "user user-7", got ${result.current.data?.name} (error: ${result.current.error})`,
        ).toBe('user user-7'),
    )
})

it('skips the request entirely without an id', async () => {
    let requests = 0
    server.use(
        http.get(endpoint(`${API_PATHS.users}/:uuid`), () => {
            requests += 1
            return HttpResponse.json({ name: 'nobody' })
        }),
    )

    const { result } = renderHook(() => useApiUser(undefined), {
        wrapper: SwrWrapper,
    })

    await waitFor(() =>
        expect(result.current.isLoading, 'the hook should have settled').toBe(
            false,
        ),
    )
    expect(requests, `expected no request, got ${requests}`).toBe(0)
})
