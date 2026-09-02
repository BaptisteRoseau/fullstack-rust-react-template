import { screen } from '@testing-library/react'

import { render } from '@/test-utils/render'

import { DriveBreadcrumbs } from './DriveBreadcrumbs'

it('marks the folder in view as the current step', () => {
    render(
        <DriveBreadcrumbs
            trail={[
                { id: 'dir-1', name: 'Invoices' },
                { id: 'dir-2', name: '2026' },
            ]}
            isComplete
        />,
    )

    expect(
        screen.getByText('2026'),
        `expected the last step to be current, got: ${document.body.textContent}`,
    ).toHaveAttribute('aria-current', 'page')
    expect(
        screen.getByRole('link', { name: 'Invoices' }),
        `expected the ancestor to stay navigable, got: ${document.body.textContent}`,
    ).toBeVisible()
})

/**
 * The backend has no ancestor-chain endpoint, so a deep link opened cold knows
 * only where it landed. The trail must say so rather than invent the parents.
 */
it('elides the ancestors it cannot know on a cold deep link', () => {
    render(
        <DriveBreadcrumbs
            trail={[{ id: 'dir-2', name: '2026' }]}
            isComplete={false}
        />,
    )

    expect(
        screen.getByText('…'),
        `expected the unknown ancestors to be elided, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByRole('link', { name: 'Home' }),
        'the root must stay reachable whatever is known of the trail',
    ).toBeVisible()
})
