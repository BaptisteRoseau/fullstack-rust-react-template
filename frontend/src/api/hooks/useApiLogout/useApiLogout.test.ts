import { act, renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { buildGetMeResponse } from '@/test-utils/fixtures/auth'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiLogout } from './useApiLogout'

it('clears the cached profile instead of refetching it', async () => {
    let profileRequests = 0

    server.use(
        http.get(endpoint(API_PATHS.me), () => {
            profileRequests += 1
            return HttpResponse.json(buildGetMeResponse())
        }),
        http.post(endpoint(API_PATHS.logout), () =>
            HttpResponse.text(null, { status: 204 }),
        ),
    )

    const { result } = renderHook(
        () => ({ profile: useApiCurrentUser(), logout: useApiLogout() }),
        { wrapper: SwrWrapper },
    )

    await waitFor(() =>
        expect(
            result.current.profile.data,
            'the profile should have loaded first',
        ).not.toBeNull(),
    )
    const requestsBeforeLogout = profileRequests

    await act(() => result.current.logout.trigger())

    await waitFor(() =>
        expect(
            result.current.profile.data,
            `expected the cached profile to be dropped, got ${JSON.stringify(result.current.profile.data)}`,
        ).toBeNull(),
    )
    expect(
        profileRequests,
        `a signed-out session must not be refetched, got ${profileRequests - requestsBeforeLogout} extra request(s)`,
    ).toBe(requestsBeforeLogout)
})
