import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { buildGetMeResponse } from '@/test-utils/fixtures/auth'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiUpdateCurrentUser } from './useApiUpdateCurrentUser'

it('refreshes the cached profile without the call site asking', async () => {
    let firstName = 'Ada'

    server.use(
        http.get(endpoint(API_PATHS.me), () =>
            HttpResponse.json(buildGetMeResponse({ firstName })),
        ),
        http.patch(endpoint(API_PATHS.me), async ({ request }) => {
            const body = (await request.json()) as { firstName: string }
            firstName = body.firstName
            return HttpResponse.json(buildGetMeResponse({ firstName }))
        }),
    )

    const { result } = renderHook(
        () => ({
            profile: useApiCurrentUser(),
            update: useApiUpdateCurrentUser(),
        }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.profile.data?.firstName,
            'the profile should have loaded first',
        ).toBe('Ada'),
    )

    await act(() =>
        result.current.update.trigger({
            firstName: 'Augusta',
            lastName: 'King',
        }),
    )

    await waitFor(() =>
        expect(
            result.current.profile.data?.firstName,
            `the mutation must invalidate the profile itself, got ${result.current.profile.data?.firstName}`,
        ).toBe('Augusta'),
    )
})
