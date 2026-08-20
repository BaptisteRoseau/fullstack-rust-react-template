import { createMiddleware } from '@mswjs/http-middleware'
import cors from 'cors'
import express from 'express'
import logger from 'pino-http'

import { env } from './src/config/env'
import { initializeDb } from './src/test-utils/mocks/db'
import { handlers } from './src/test-utils/mocks/handlers'
import { FORWARDED_COOKIE_HEADER } from './src/test-utils/mocks/utils'

const app = express()

app.use(cors({ origin: env.APP_URL, credentials: true }))
app.use(express.json())
app.use(logger())

app.use((request, _response, next) => {
    request.headers[FORWARDED_COOKIE_HEADER] = request.headers.cookie ?? ''
    next()
})

app.use(createMiddleware(...handlers))

void initializeDb().then(() => {
    app.listen(env.APP_MOCK_API_PORT, () => {
        console.log(
            `Mock API server started at http://localhost:${env.APP_MOCK_API_PORT}`,
        )
    })
})
