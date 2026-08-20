import { useLingui } from '@lingui/react/macro'
import { Navigate, useLocation } from 'react-router'

import { useCurrentUser } from '@/api/service/auth'
import { Spinner } from '@/design-system/Spinner'
import { PATHS } from '@/router/constants'

import styles from './protected-route.module.scss'

export function ProtectedRoute({ children }: { children: React.ReactNode }) {
    const { t } = useLingui()
    const { data: user, isLoading } = useCurrentUser()
    const location = useLocation()

    if (isLoading) {
        return (
            <div className={styles.loading}>
                <Spinner size="lg" label={t`Loading`} />
            </div>
        )
    }

    if (!user) {
        return (
            <Navigate
                to={`${PATHS.login}?redirect=${encodeURIComponent(location.pathname)}`}
                replace
            />
        )
    }

    return children
}
