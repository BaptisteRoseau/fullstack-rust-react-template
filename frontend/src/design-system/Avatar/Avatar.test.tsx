import { render, screen } from '@testing-library/react'

import { Avatar } from './Avatar'

it('renders the initials of the name', () => {
    render(<Avatar name="Ada Lovelace" />)

    expect(
        screen.getByText('AL'),
        `expected "AL", got "${document.body.textContent}"`,
    ).toBeVisible()
})
