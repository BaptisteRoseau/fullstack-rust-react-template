import { i18n } from '@lingui/core'

export type Locale = 'en' | 'fr'

export const defaultLocale: Locale = 'en'

export const localeLabels: Record<Locale, string> = {
    en: 'English',
    fr: 'Français',
}

export async function loadLocale(locale: Locale): Promise<void> {
    const { messages } = await import(`./locales/${locale}/messages`)
    i18n.load(locale, messages)
    i18n.activate(locale)
}

export { i18n }
