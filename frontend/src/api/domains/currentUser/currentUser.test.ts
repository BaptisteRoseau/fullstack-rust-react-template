import { http, HttpResponse } from 'msw'

import { isApiError } from '@/api/errors'
import { buildGetMeResponse } from '@/test-utils/fixtures/auth'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'

import { fetchCurrentUser, updateCurrentUser } from './currentUser'

it('returns the signed-in user as a domain object', async () => {
    server.use(
        http.get(endpoint(API_PATHS.me), () =>
            HttpResponse.json(buildGetMeResponse({ email: 'ada@example.com' })),
        ),
    )

    const user = await fetchCurrentUser()

    expect(user?.email, `expected ada@example.com, got ${user?.email}`).toBe(
        'ada@example.com',
    )
    expect(
        user?.createdAt instanceof Date,
        'the fetcher must hand back the converted shape',
    ).toBe(true)
})

it('answers null rather than throwing when the session is gone', async () => {
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

    const user = await fetchCurrentUser()

    expect(
        user,
        `signed out is an answer, not a failure; got ${JSON.stringify(user)}`,
    ).toBeNull()
})

it('lets a genuine failure through', async () => {
    server.use(
        http.get(endpoint(API_PATHS.me), () =>
            HttpResponse.json(
                { error: 'Boom', id: 'UNEXPECTED' },
                { status: 500 },
            ),
        ),
    )

    const error = await fetchCurrentUser().catch((thrown) => thrown)

    expect(isApiError(error), `expected an ApiError, got ${error}`).toBe(true)
    expect(error.status, `expected 500, got ${error.status}`).toBe(500)
})

it('sends only the owned fields when updating the profile', async () => {
    let body: unknown
    server.use(
        http.patch(endpoint(API_PATHS.me), async ({ request }) => {
            body = await request.json()
            return HttpResponse.json(
                buildGetMeResponse({ firstName: 'Augusta' }),
            )
        }),
    )

    const user = await updateCurrentUser({
        firstName: 'Augusta',
        lastName: 'King',
    })

    expect(body, `unexpected request body: ${JSON.stringify(body)}`).toEqual({
        firstName: 'Augusta',
        lastName: 'King',
    })
    expect(
        user.firstName,
        `expected the updated profile back, got ${user.firstName}`,
    ).toBe('Augusta')
})
