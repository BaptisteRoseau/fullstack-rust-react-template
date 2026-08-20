import { render, screen } from '@testing-library/react'

import { Spinner } from './Spinner'

it('exposes its label to assistive technology', () => {
    render(<Spinner label="Loading users" />)

    const status = screen.getByRole('status')

    expect(
        status,
        `expected the accessible name "Loading users", got "${status.getAttribute('aria-label')}"`,
    ).toHaveAccessibleName('Loading users')
})
