import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'

import { Button } from './Button'

it('calls onClick when pressed', async () => {
    const onClick = vi.fn()
    render(<Button onClick={onClick}>Save</Button>)

    await userEvent.click(screen.getByRole('button', { name: 'Save' }))

    expect(
        onClick,
        `expected 1 call, got ${onClick.mock.calls.length}`,
    ).toHaveBeenCalledTimes(1)
})

it('does not fire when disabled', async () => {
    const onClick = vi.fn()
    render(
        <Button onClick={onClick} disabled>
            Save
        </Button>,
    )

    await userEvent.click(screen.getByRole('button', { name: 'Save' }))

    expect(
        onClick,
        `expected no calls, got ${onClick.mock.calls.length}`,
    ).not.toHaveBeenCalled()
})
