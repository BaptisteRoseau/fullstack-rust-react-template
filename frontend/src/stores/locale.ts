import { create } from 'zustand'

import { LOCALE_STORAGE_KEY } from '@/constants/storage'
import { defaultLocale, loadLocale, locales, type Locale } from '@/i18n'

type LocaleStore = {
    locale: Locale
    setLocale: (locale: Locale) => Promise<void>
}

export function storedLocale(): Locale {
    const stored = window.localStorage.getItem(LOCALE_STORAGE_KEY)
    return locales.includes(stored as Locale)
        ? (stored as Locale)
        : defaultLocale
}

export const useLocale = create<LocaleStore>((set) => ({
    locale: storedLocale(),
    setLocale: async (locale) => {
        await loadLocale(locale)
        window.localStorage.setItem(LOCALE_STORAGE_KEY, locale)
        set({ locale })
    },
}))
