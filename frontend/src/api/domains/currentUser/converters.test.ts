import { fromGetMeResponse, toPatchMeRequest } from './converters'

const wireCurrentUser = {
    id: 'user-1',
    email: 'ada@example.com',
    firstName: 'Ada',
    lastName: 'Lovelace',
    role: 'USER',
    teamId: 'team-1',
    createdAt: Date.UTC(2026, 0, 15),
}

it('reads createdAt as milliseconds, not seconds', () => {
    const user = fromGetMeResponse(wireCurrentUser)

    expect(
        user.createdAt.toISOString(),
        `expected 2026-01-15, got ${user.createdAt.toISOString()}`,
    ).toBe('2026-01-15T00:00:00.000Z')
})

it('carries the identity fields across unchanged', () => {
    const user = fromGetMeResponse(wireCurrentUser)

    expect(user.email, `expected the email, got ${user.email}`).toBe(
        'ada@example.com',
    )
    expect(user.role, `expected the role, got ${user.role}`).toBe('USER')
})

it('sends only the fields the user owns', () => {
    const request = toPatchMeRequest({
        firstName: 'Augusta',
        lastName: 'King',
    })

    expect(
        request,
        `the request must not carry read-only fields: ${JSON.stringify(request)}`,
    ).toEqual({ firstName: 'Augusta', lastName: 'King' })
})
