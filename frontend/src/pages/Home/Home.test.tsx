import { screen } from '@testing-library/react'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { buildCurrentUser } from '@/test-utils/fixtures/auth'
import { render } from '@/test-utils/render'

import { Home } from './Home'

vi.mock('@/api/hooks/useApiCurrentUser')

function mockCurrentUser(data: ReturnType<typeof buildCurrentUser> | null) {
    vi.mocked(useApiCurrentUser).mockReturnValue({
        data,
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    })
}

it('renders the hero and the call to action when signed out', () => {
    mockCurrentUser(null)

    render(<Home />)

    expect(
        screen.getByRole('heading', {
            name: /cloud storage that secures itself/i,
        }),
        `expected the hero heading, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByRole('link', { name: 'Get started' }),
        `expected a "Get started" link, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('sends a signed-in visitor to their drive', () => {
    mockCurrentUser(buildCurrentUser())

    render(<Home />)

    const driveLink = screen.getByRole('link', { name: 'Open your drive' })

    expect(
        driveLink,
        `expected an "Open your drive" link, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        driveLink,
        `expected the drive link to point at /drive, got: ${driveLink.getAttribute('href')}`,
    ).toHaveAttribute('href', '/drive')
})
