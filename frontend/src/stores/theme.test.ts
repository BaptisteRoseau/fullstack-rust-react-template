import { THEME_STORAGE_KEY } from '@/constants/storage'

type ChangeListener = () => void

const listeners = new Set<ChangeListener>()
let systemPrefersDark = false

function stubColorScheme() {
    listeners.clear()
    window.localStorage.clear()
    vi.stubGlobal('matchMedia', (query: string) => ({
        matches: systemPrefersDark && query === '(prefers-color-scheme: dark)',
        addEventListener: (_: string, listener: ChangeListener) =>
            listeners.add(listener),
        removeEventListener: (_: string, listener: ChangeListener) =>
            listeners.delete(listener),
    }))
}

async function loadStore() {
    vi.resetModules()
    return (await import('./theme')).useTheme
}

afterEach(() => {
    vi.unstubAllGlobals()
    window.localStorage.clear()
})

it('starts on the system theme and follows a dark system preference', async () => {
    systemPrefersDark = true
    stubColorScheme()

    const useTheme = await loadStore()

    expect(
        useTheme.getState().theme,
        `expected the system theme, got "${useTheme.getState().theme}"`,
    ).toBe('system')
    expect(
        useTheme.getState().resolvedTheme,
        `expected dark to be resolved, got "${useTheme.getState().resolvedTheme}"`,
    ).toBe('dark')
})

it('falls back to the light theme when the system preference cannot be read', async () => {
    vi.stubGlobal('matchMedia', undefined)
    window.localStorage.clear()

    const useTheme = await loadStore()
    useTheme.getState().setTheme('system')

    expect(
        useTheme.getState().resolvedTheme,
        `expected light without matchMedia, got "${useTheme.getState().resolvedTheme}"`,
    ).toBe('light')
    expect(
        document.documentElement.dataset.theme,
        `expected light on the document, got "${document.documentElement.dataset.theme}"`,
    ).toBe('light')
})

it('follows the system while on system, and stops once a theme is chosen', async () => {
    systemPrefersDark = false
    stubColorScheme()

    const useTheme = await loadStore()
    systemPrefersDark = true
    listeners.forEach((listener) => listener())

    expect(
        useTheme.getState().resolvedTheme,
        `expected the store to follow the system to dark, got "${useTheme.getState().resolvedTheme}"`,
    ).toBe('dark')

    useTheme.getState().setTheme('light')
    systemPrefersDark = false
    listeners.forEach((listener) => listener())

    expect(
        useTheme.getState().resolvedTheme,
        `expected an explicit light choice to survive, got "${useTheme.getState().resolvedTheme}"`,
    ).toBe('light')
})

it('restores the stored preference over the system default', async () => {
    systemPrefersDark = true
    stubColorScheme()
    window.localStorage.setItem(THEME_STORAGE_KEY, 'light')

    const useTheme = await loadStore()

    expect(
        useTheme.getState().theme,
        `expected the stored light preference, got "${useTheme.getState().theme}"`,
    ).toBe('light')
    expect(
        useTheme.getState().resolvedTheme,
        `expected light despite a dark system, got "${useTheme.getState().resolvedTheme}"`,
    ).toBe('light')
})
