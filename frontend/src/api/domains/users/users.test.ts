import { http, HttpResponse } from 'msw'

import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'

import { fetchUser } from './users'

it('fetches a user by id', async () => {
    server.use(
        http.get(endpoint(`${API_PATHS.users}/:uuid`), () =>
            HttpResponse.json({ name: 'Ada Lovelace' }),
        ),
    )

    const user = await fetchUser('user-1')

    expect(user.name, `expected Ada Lovelace, got ${user.name}`).toBe(
        'Ada Lovelace',
    )
})
