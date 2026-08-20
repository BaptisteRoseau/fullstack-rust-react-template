import { create } from 'zustand'

import { THEME_STORAGE_KEY } from '@/constants/storage'

export type Theme = 'light' | 'dark'

type ThemeStore = {
    theme: Theme
    setTheme: (theme: Theme) => void
    toggleTheme: () => void
}

function readStoredTheme(): Theme {
    return window.localStorage.getItem(THEME_STORAGE_KEY) === 'dark'
        ? 'dark'
        : 'light'
}

function applyTheme(theme: Theme) {
    window.localStorage.setItem(THEME_STORAGE_KEY, theme)
    document.documentElement.dataset.theme = theme
}

export const useTheme = create<ThemeStore>((set, get) => ({
    theme: readStoredTheme(),
    setTheme: (theme) => {
        applyTheme(theme)
        set({ theme })
    },
    toggleTheme: () =>
        get().setTheme(get().theme === 'dark' ? 'light' : 'dark'),
}))
