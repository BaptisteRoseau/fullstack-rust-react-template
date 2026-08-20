import { screen } from '@testing-library/react'

import { useCurrentUser } from '@/api/service/auth'
import { buildCurrentUser } from '@/test-utils/fixtures/auth'
import { render } from '@/test-utils/render'

import { AppHeader } from './AppHeader'

vi.mock('@/api/service/auth')

function mockCurrentUser(data: ReturnType<typeof buildCurrentUser> | null) {
    vi.mocked(useCurrentUser).mockReturnValue({
        data,
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    })
}

it('shows the log in and register buttons when signed out', () => {
    mockCurrentUser(null)

    render(<AppHeader />)

    expect(
        screen.getByRole('link', { name: 'Log in' }),
        `expected a "Log in" link, got: ${document.body.textContent}`,
    ).toBeVisible()
    expect(
        screen.getByRole('link', { name: 'Register' }),
        `expected a "Register" link, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('shows the user name when signed in', () => {
    const user = buildCurrentUser({ firstName: 'Ada', lastName: 'Lovelace' })
    mockCurrentUser(user)

    render(<AppHeader />)

    expect(
        screen.getByRole('button', { name: /ada lovelace/i }),
        `expected the account trigger for ${user.firstName}, got: ${document.body.textContent}`,
    ).toBeVisible()
})
