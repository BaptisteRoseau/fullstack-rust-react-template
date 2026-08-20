import { screen } from '@testing-library/react'

import { useApiKeys } from '@/api/service/apiKeys'
import { buildApiKey } from '@/test-utils/fixtures/apiKeys'
import { render } from '@/test-utils/render'

import { ApiKeys } from './ApiKeys'

vi.mock('@/api/service/apiKeys')

it('lists the api keys', () => {
    const apiKey = buildApiKey({ name: 'CI deploy key' })
    vi.mocked(useApiKeys).mockReturnValue({
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
    vi.mocked(useApiKeys).mockReturnValue({
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
