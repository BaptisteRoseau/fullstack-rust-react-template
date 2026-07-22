import { I18nProvider } from '@lingui/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import * as React from 'react'
import { ErrorBoundary } from 'react-error-boundary'
import { HelmetProvider } from 'react-helmet-async'

import { MainErrorFallback } from '@/components/errors/main'
import { Notifications } from '@/components/ui/notifications'
import { Spinner } from '@/components/ui/spinner'
import { i18n } from '@/i18n'
import { AuthLoader } from '@/lib/auth'
import { queryConfig } from '@/lib/react-query'

type AppProviderProps = {
    children: React.ReactNode
}

export const AppProvider = ({ children }: AppProviderProps) => {
    const [queryClient] = React.useState(
        () =>
            new QueryClient({
                defaultOptions: queryConfig,
            }),
    )

    return (
        <React.Suspense
            fallback={
                <div className="flex h-screen w-screen items-center justify-center">
                    <Spinner size="xl" />
                </div>
            }
        >
            <I18nProvider i18n={i18n}>
                <ErrorBoundary FallbackComponent={MainErrorFallback}>
                    <HelmetProvider>
                        <QueryClientProvider client={queryClient}>
                            {import.meta.env.DEV && <ReactQueryDevtools />}
                            <Notifications />
                            <AuthLoader
                                renderLoading={() => (
                                    <div className="flex h-screen w-screen items-center justify-center">
                                        <Spinner size="xl" />
                                    </div>
                                )}
                                // Resolving the current user must never take the
                                // whole app down. On failure we render the public
                                // view, exactly as for an anonymous visitor; the
                                // api client has already reported the error as a
                                // notification.
                                renderError={() => <>{children}</>}
                            >
                                {children}
                            </AuthLoader>
                        </QueryClientProvider>
                    </HelmetProvider>
                </ErrorBoundary>
            </I18nProvider>
        </React.Suspense>
    )
}
