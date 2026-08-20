import { http, HttpResponse } from 'msw'

import { resetDb, seedDb } from '../db'
import { endpoint } from '../utils'

export const RESET_ENDPOINT = '/api/__reset'

export const resetHandlers = [
    http.post(endpoint(RESET_ENDPOINT), async () => {
        resetDb()
        await seedDb()
        return HttpResponse.text(null, { status: 204 })
    }),
]
