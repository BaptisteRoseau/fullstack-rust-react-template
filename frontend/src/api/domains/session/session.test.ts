import { http, HttpResponse } from 'msw'

import { env } from '@/config/env'
import { API_PATHS, endpoint } from '@/test-utils/mocks/utils'
import { server } from '@/test-utils/server'

import { loginUrl, logout, registerUrl } from './session'

it('posts a logout and resolves without a payload', async () => {
    let loggedOut = false
    server.use(
        http.post(endpoint(API_PATHS.logout), () => {
            loggedOut = true
            return HttpResponse.text(null, { status: 204 })
        }),
    )

    await logout()

    expect(loggedOut, 'the logout request must reach the backend').toBe(true)
})

it('builds the OIDC entry points against the backend', () => {
    expect(loginUrl(), `unexpected login url: ${loginUrl()}`).toBe(
        `${env.API_URL}/api/auth/login`,
    )
    expect(registerUrl(), `unexpected register url: ${registerUrl()}`).toBe(
        `${env.API_URL}/api/auth/register`,
    )
})

it('encodes the return path as a query parameter', () => {
    const url = loginUrl('/user/api-keys')

    expect(url, `expected an encoded redirect, got ${url}`).toBe(
        `${env.API_URL}/api/auth/login?redirect=%2Fuser%2Fapi-keys`,
    )
})
