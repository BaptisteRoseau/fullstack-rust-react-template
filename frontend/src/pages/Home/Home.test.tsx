import { screen } from '@testing-library/react'

import { useCurrentUser } from '@/api/service/auth'
import { render } from '@/test-utils/render'

import { Home } from './Home'

vi.mock('@/api/service/auth')

it('renders the hero and the call to action when signed out', () => {
    vi.mocked(useCurrentUser).mockReturnValue({
        data: null,
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    })

    render(<Home />)

    expect(
        screen.getByRole('heading', {
            name: /ship a fullstack app, not a toolchain/i,
        }),
        `expected the hero heading, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByRole('link', { name: 'Get started' }),
        `expected a "Get started" link, got: ${document.body.textContent}`,
    ).toBeVisible()
})
