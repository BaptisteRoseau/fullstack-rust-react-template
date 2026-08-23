import { renderHook } from '@testing-library/react'

import { I18nWrapper } from '@/test-utils/wrappers'

import {
    ApiError,
    isApiError,
    matchApiError,
    toApiError,
    toNetworkError,
    useApiErrorMessage,
} from './errors'

it('keeps the id and status of a documented error body', () => {
    const response = new Response(null, { status: 404 })
    const error = toApiError({ error: 'Not found.', id: 'NOT_FOUND' }, response)

    expect(error.id, `expected NOT_FOUND, got ${error.id}`).toBe('NOT_FOUND')
    expect(error.status, `expected 404, got ${error.status}`).toBe(404)
    expect(isApiError(error), 'toApiError must produce an ApiError').toBe(true)
})

it('falls back to PARSE when the body is not an error response', () => {
    const response = new Response(null, { status: 500 })

    const unparseable = toApiError('<html>gateway timeout</html>', response)
    const unknownId = toApiError({ error: 'boom', id: 'TEAPOT' }, response)

    expect(
        unparseable.id,
        `expected PARSE for an HTML body, got ${unparseable.id}`,
    ).toBe('PARSE')
    expect(
        unknownId.id,
        `expected PARSE for an id outside the contract, got ${unknownId.id}`,
    ).toBe('PARSE')
})

it('reports a failed fetch as NETWORK with no status', () => {
    const error = toNetworkError(new TypeError('Failed to fetch'))

    expect(error.id, `expected NETWORK, got ${error.id}`).toBe('NETWORK')
    expect(error.status, `expected 0, got ${error.status}`).toBe(0)
})

it('routes matchApiError on the cause, not the status code', () => {
    const expired = new ApiError('expired', 401, 'TOKEN_EXPIRED', null)
    const signedOut = new ApiError('signed out', 401, 'UNAUTHORIZED', null)

    const branch = (error: unknown) =>
        matchApiError(error, {
            TOKEN_EXPIRED: () => 'refresh',
            default: () => 'sign-in',
        })

    expect(branch(expired), 'TOKEN_EXPIRED must take its own branch').toBe(
        'refresh',
    )
    expect(
        branch(signedOut),
        'an unhandled id must fall through to default despite sharing the status',
    ).toBe('sign-in')
    expect(
        branch(new Error('not from the api')),
        'a foreign error must fall through to default',
    ).toBe('sign-in')
})

it('translates a known cause and uses the fallback for foreign errors', () => {
    const { result } = renderHook(() => useApiErrorMessage(), {
        wrapper: I18nWrapper,
    })

    const known = result.current(
        new ApiError('Not found.', 404, 'NOT_FOUND', null),
        'Could not load the key',
    )
    const foreign = result.current(new Error('boom'), 'Could not load the key')

    expect(
        known,
        `the backend's own string must not reach the interface, got "${known}"`,
    ).toBe('This resource no longer exists.')
    expect(foreign, `expected the call-site fallback, got "${foreign}"`).toBe(
        'Could not load the key',
    )
})
