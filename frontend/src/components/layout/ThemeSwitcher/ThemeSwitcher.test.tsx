import { screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'

import { THEME_STORAGE_KEY } from '@/constants/storage'
import { useTheme } from '@/stores/theme'
import { render } from '@/test-utils/render'

import { ThemeSwitcher } from './ThemeSwitcher'

beforeEach(() => {
    useTheme.getState().setTheme('system')
})

function switcher(name: RegExp) {
    return screen.getByRole('button', { name })
}

it('starts on the system theme and cycles through light and dark', async () => {
    render(<ThemeSwitcher />)

    expect(
        switcher(/^Theme: system/),
        `expected the system theme first, got: ${document.body.textContent}`,
    ).toBeVisible()

    await userEvent.click(switcher(/^Theme: system/))
    expect(
        useTheme.getState().theme,
        `expected light after one click, got "${useTheme.getState().theme}"`,
    ).toBe('light')

    await userEvent.click(switcher(/^Theme: light/))
    expect(
        useTheme.getState().theme,
        `expected dark after two clicks, got "${useTheme.getState().theme}"`,
    ).toBe('dark')

    await userEvent.click(switcher(/^Theme: dark/))
    expect(
        useTheme.getState().theme,
        `expected system again after three clicks, got "${useTheme.getState().theme}"`,
    ).toBe('system')
})

it('applies and persists the chosen theme', async () => {
    render(<ThemeSwitcher />)

    await userEvent.click(switcher(/^Theme: system/))
    await userEvent.click(switcher(/^Theme: light/))

    expect(
        document.documentElement.dataset.theme,
        `expected the dark theme on the document, got "${document.documentElement.dataset.theme}"`,
    ).toBe('dark')
    expect(
        window.localStorage.getItem(THEME_STORAGE_KEY),
        `expected "dark" to be stored, got "${window.localStorage.getItem(THEME_STORAGE_KEY)}"`,
    ).toBe('dark')
})

it('names the next theme in a tooltip', async () => {
    render(<ThemeSwitcher />)

    expect(
        switcher(/^Theme: system/),
        `expected the system tooltip, got: ${switcher(/^Theme: system/).title}`,
    ).toHaveAttribute('title', 'Switch to the light theme')

    await userEvent.click(switcher(/^Theme: system/))

    expect(
        switcher(/^Theme: light/),
        `expected the light tooltip, got: ${switcher(/^Theme: light/).title}`,
    ).toHaveAttribute('title', 'Switch to the dark theme')
})
