import { SWRConfig } from 'swr'

import { apiFetch } from '@/api/client'

export function SwrWrapper({ children }: { children: React.ReactNode }) {
    return (
        <SWRConfig
            value={{
                fetcher: apiFetch,
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
