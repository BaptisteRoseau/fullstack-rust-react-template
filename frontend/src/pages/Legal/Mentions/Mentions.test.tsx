import { screen } from '@testing-library/react'

import { render } from '@/test-utils/render'

import { Mentions } from './Mentions'

it('renders the heading and the proof-of-concept disclaimer', () => {
    render(<Mentions />)

    expect(
        screen.getByRole('heading', { name: 'Legal mentions', level: 1 }),
        `expected the "Legal mentions" heading, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByText(/placeholder for this proof of concept/i),
        `expected the demo disclaimer, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('names the fictional publisher', () => {
    render(<Mentions />)

    expect(
        screen.getByRole('heading', { name: 'Publisher', level: 2 }),
        `expected the "Publisher" section, got: ${document.body.textContent}`,
    ).toBeVisible()
    const mentionsOfThePublisher = screen.getAllByText(/driftbox sas/i)

    expect(
        mentionsOfThePublisher.length,
        `expected the fictional publisher to be named, got ${mentionsOfThePublisher.length} mentions in: ${document.body.textContent}`,
    ).toBeGreaterThan(0)
})
