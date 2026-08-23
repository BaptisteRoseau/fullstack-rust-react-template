import { I18nProvider } from '@lingui/react'
import { ErrorBoundary } from 'react-error-boundary'
import { SWRConfig } from 'swr'

import { ErrorFallback } from '@/components/errors/ErrorFallback'
import { Notifications } from '@/components/notifications/Notifications'
import { i18n } from '@/i18n'

export function Context({ children }: { children: React.ReactNode }) {
    return (
        <ErrorBoundary FallbackComponent={ErrorFallback}>
            <I18nProvider i18n={i18n}>
                <SWRConfig
                    value={{
                        revalidateOnFocus: false,
                        shouldRetryOnError: false,
                    }}
                >
                    {children}
                    <Notifications />
                </SWRConfig>
            </I18nProvider>
        </ErrorBoundary>
    )
}
