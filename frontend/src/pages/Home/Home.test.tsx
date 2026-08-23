import { screen } from '@testing-library/react'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { render } from '@/test-utils/render'

import { Home } from './Home'

vi.mock('@/api/hooks/useApiCurrentUser')

it('renders the hero and the call to action when signed out', () => {
    vi.mocked(useApiCurrentUser).mockReturnValue({
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
