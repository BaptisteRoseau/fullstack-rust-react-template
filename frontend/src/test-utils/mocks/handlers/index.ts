import { apiKeyHandlers } from './apiKeys'
import { authHandlers } from './auth'
import { resetHandlers } from './reset'
import { userHandlers } from './users'

export const handlers = [
    ...authHandlers,
    ...apiKeyHandlers,
    ...userHandlers,
    ...resetHandlers,
]
