import { apiKeyHandlers } from './apiKeys'
import { authHandlers } from './auth'
import { driveHandlers } from './drive'
import { resetHandlers } from './reset'
import { userHandlers } from './users'

export const handlers = [
    ...authHandlers,
    ...apiKeyHandlers,
    ...driveHandlers,
    ...userHandlers,
    ...resetHandlers,
]
