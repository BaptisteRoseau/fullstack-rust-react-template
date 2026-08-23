import { fromGetUserResponse } from './converters'

it('builds the domain user from the wire response', () => {
    const user = fromGetUserResponse({ name: 'Ada Lovelace' })

    expect(user.name, `expected the name, got ${user.name}`).toBe(
        'Ada Lovelace',
    )
})
