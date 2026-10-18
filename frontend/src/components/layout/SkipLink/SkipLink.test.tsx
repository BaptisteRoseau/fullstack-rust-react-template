import { screen } from '@testing-library/react'

import { render } from '@/test-utils/render'

import { SkipLink } from './SkipLink'

it('links to the target element', () => {
    render(<SkipLink targetId="main" />)

    const link = screen.getByRole('link', { name: 'Skip to main content' })
    expect(
        link,
        `expected the link to target #main, got "${link.getAttribute('href')}"`,
    ).toHaveAttribute('href', '#main')
})
