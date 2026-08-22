import { create } from 'zustand'

import { THEME_STORAGE_KEY } from '@/constants/storage'

export type Theme = 'system' | 'light' | 'dark'
export type ResolvedTheme = Exclude<Theme, 'system'>

export const themes: Theme[] = ['system', 'light', 'dark']

const DARK_COLOR_SCHEME = '(prefers-color-scheme: dark)'

type ThemeStore = {
    theme: Theme
    resolvedTheme: ResolvedTheme
    setTheme: (theme: Theme) => void
    cycleTheme: () => void
}

function darkColorScheme(): MediaQueryList | null {
    return typeof window.matchMedia === 'function'
        ? window.matchMedia(DARK_COLOR_SCHEME)
        : null
}

function resolveTheme(theme: Theme): ResolvedTheme {
    if (theme !== 'system') {
        return theme
    }

    return darkColorScheme()?.matches ? 'dark' : 'light'
}

function readStoredTheme(): Theme {
    const stored = window.localStorage.getItem(THEME_STORAGE_KEY)
    return themes.includes(stored as Theme) ? (stored as Theme) : 'system'
}

function applyTheme(theme: Theme): ResolvedTheme {
    const resolved = resolveTheme(theme)
    window.localStorage.setItem(THEME_STORAGE_KEY, theme)
    document.documentElement.dataset.theme = resolved
    return resolved
}

function nextTheme(theme: Theme): Theme {
    return themes[(themes.indexOf(theme) + 1) % themes.length]
}

const initialTheme = readStoredTheme()

export const useTheme = create<ThemeStore>((set, get) => ({
    theme: initialTheme,
    resolvedTheme: resolveTheme(initialTheme),
    setTheme: (theme) => set({ theme, resolvedTheme: applyTheme(theme) }),
    cycleTheme: () => get().setTheme(nextTheme(get().theme)),
}))

darkColorScheme()?.addEventListener('change', () => {
    if (useTheme.getState().theme === 'system') {
        useTheme.getState().setTheme('system')
    }
})
