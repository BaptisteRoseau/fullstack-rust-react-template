import { screen } from '@testing-library/react'

import { render } from '@/test-utils/render'

import { Terms } from './Terms'

it('renders the heading and the proof-of-concept disclaimer', () => {
    render(<Terms />)

    expect(
        screen.getByRole('heading', { name: 'Terms of Service', level: 1 }),
        `expected the "Terms of Service" heading, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByText(/placeholder for this proof of concept/i),
        `expected the demo disclaimer, got: ${document.body.textContent}`,
    ).toBeVisible()
})
