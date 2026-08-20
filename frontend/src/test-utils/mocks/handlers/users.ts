import { http, HttpResponse } from 'msw'

import { USERS_ENDPOINT } from '@/api/users'

import { db } from '../db'
import { endpoint, networkDelay } from '../utils'

export const userHandlers = [
    http.get(endpoint(`${USERS_ENDPOINT}/:userId`), async ({ params }) => {
        await networkDelay()
        const user = db.user.findFirst({
            where: { id: { equals: String(params.userId) } },
        })
        if (!user) {
            return HttpResponse.json(
                { id: 'NOT_FOUND', error: 'The user does not exist' },
                { status: 404 },
            )
        }
        return HttpResponse.json({ name: `${user.firstName} ${user.lastName}` })
    }),
]
