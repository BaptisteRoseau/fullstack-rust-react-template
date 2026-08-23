import { screen } from '@testing-library/react'

import { useApiApiKeys } from '@/api/hooks/useApiApiKeys'
import { buildApiKey } from '@/test-utils/fixtures/apiKeys'
import { render } from '@/test-utils/render'

import { ApiKeys } from './ApiKeys'

vi.mock('@/api/hooks/useApiApiKeys')

it('lists the api keys', () => {
    const apiKey = buildApiKey({ name: 'CI deploy key' })
    vi.mocked(useApiApiKeys).mockReturnValue({
        data: [apiKey],
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    })

    render(<ApiKeys />)

    expect(
        screen.getByText('CI deploy key'),
        `expected the key row, got: ${document.body.textContent}`,
    ).toBeVisible()
})

it('shows the empty state when there is no key', () => {
    vi.mocked(useApiApiKeys).mockReturnValue({
        data: [],
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    })

    render(<ApiKeys />)

    expect(
        screen.getByText('You have no API key yet.'),
        `expected the empty state, got: ${document.body.textContent}`,
    ).toBeVisible()
})
