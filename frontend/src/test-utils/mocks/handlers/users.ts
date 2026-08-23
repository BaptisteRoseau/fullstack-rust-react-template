import { http, HttpResponse } from 'msw'

import type { GetUserResponse } from '@/api/generated'

import { db } from '../db'
import { API_PATHS, endpoint, networkDelay } from '../utils'

export const userHandlers = [
    http.get(endpoint(`${API_PATHS.users}/:userId`), async ({ params }) => {
        await networkDelay()
        const user = db.user.findFirst((query) =>
            query.where({ id: String(params.userId) }),
        )
        if (!user) {
            return HttpResponse.json(
                { id: 'NOT_FOUND', error: 'The user does not exist' },
                { status: 404 },
            )
        }
        return HttpResponse.json<GetUserResponse>({
            name: `${user.firstName} ${user.lastName}`,
        })
    }),
]
