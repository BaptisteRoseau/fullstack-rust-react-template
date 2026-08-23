import {
    fromCreateApiKeyResponse,
    fromGetApiKeyResponse,
    toCreateApiKeyRequest,
} from './converters'

const wireApiKey = {
    id: 'key-1',
    name: 'CI deploy key',
    permissions: ['read', 'write'],
    createdAt: '2026-01-15T09:30:00Z',
}

it('turns the wire timestamp into a Date', () => {
    const apiKey = fromGetApiKeyResponse(wireApiKey)

    expect(
        apiKey.createdAt instanceof Date,
        `expected a Date, got ${typeof apiKey.createdAt}`,
    ).toBe(true)
    expect(
        apiKey.createdAt.toISOString(),
        `expected the wire instant to survive, got ${apiKey.createdAt.toISOString()}`,
    ).toBe('2026-01-15T09:30:00.000Z')
})

it('narrows the wire string list to known permissions', () => {
    const apiKey = fromGetApiKeyResponse(wireApiKey)

    expect(
        apiKey.permissions,
        `expected the two known permissions, got ${apiKey.permissions.join()}`,
    ).toEqual(['read', 'write'])
})

it('drops a permission the frontend does not know rather than failing', () => {
    const apiKey = fromGetApiKeyResponse({
        ...wireApiKey,
        permissions: ['read', 'teleport'],
    })

    expect(
        apiKey.permissions,
        `an unknown permission must not blank the list, got ${apiKey.permissions.join()}`,
    ).toEqual(['read'])
})

it('renames the wire key to secret', () => {
    const created = fromCreateApiKeyResponse({
        ...wireApiKey,
        key: 'sk_live_1234',
    })

    expect(
        created.secret,
        `expected the raw key under "secret", got ${created.secret}`,
    ).toBe('sk_live_1234')
    expect(
        'key' in created,
        'the wire name must not leak into the domain object',
    ).toBe(false)
})

it('sends a plain array back on the wire', () => {
    const request = toCreateApiKeyRequest({
        name: 'CI deploy key',
        permissions: ['read', 'admin'],
    })

    expect(
        request,
        `unexpected request body: ${JSON.stringify(request)}`,
    ).toEqual({ name: 'CI deploy key', permissions: ['read', 'admin'] })
})
