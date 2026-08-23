import { I18nProvider } from '@lingui/react'
import { SWRConfig } from 'swr'

import { i18n } from '@/i18n'

export function SwrWrapper({ children }: { children: React.ReactNode }) {
    return (
        <SWRConfig
            value={{
                provider: () => new Map(),
                dedupingInterval: 0,
                revalidateOnFocus: false,
                shouldRetryOnError: false,
            }}
        >
            {children}
        </SWRConfig>
    )
}

export function I18nWrapper({ children }: { children: React.ReactNode }) {
    return <I18nProvider i18n={i18n}>{children}</I18nProvider>
}
