import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactNode, useState, Suspense } from 'react'
import { ErrorBoundary } from 'react-error-boundary'
import { HelmetProvider } from 'react-helmet-async'

import { MainErrorFallback } from './components/errors/main'
import { Notifications } from './components/ui/notifications'
import { Spinner } from './components/ui/spinner'
import { AuthLoader } from './lib/auth'
import { queryConfig } from './lib/react-query'

interface AppProviderProps {
    children: ReactNode
}

function AppProvider({ children }: AppProviderProps) {
    const [queryClient] = useState(
        () =>
            new QueryClient({
                defaultOptions: queryConfig,
            })
    )

    return (
        <Suspense
            fallback={
                <div className="flex h-screen w-screen items-center justify-center">
                    <Spinner size="xl" />
                </div>
            }
        >
            <ErrorBoundary FallbackComponent={MainErrorFallback}>
                <HelmetProvider>
                    <QueryClientProvider client={queryClient}>
                        <Notifications />
                        <AuthLoader
                            renderLoading={() => (
                                <div className="flex h-screen w-screen items-center justify-center">
                                    <Spinner size="xl" />
                                </div>
                            )}
                        >
                            {children}
                        </AuthLoader>
                    </QueryClientProvider>
                </HelmetProvider>
            </ErrorBoundary>
        </Suspense>
    )
}

export default AppProvider
