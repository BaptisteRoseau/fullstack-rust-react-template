import { useLingui } from '@lingui/react/macro'
import { Outlet } from 'react-router'

import { Logo } from '@/components/layout/Logo'
import { Card } from '@/design-system/Card'

import styles from './auth-layout.module.scss'

export function AuthLayout() {
    const { t } = useLingui()

    return (
        <div className={styles.layout}>
            <Logo label={t`Driftbox`} />
            <Card className={styles.card}>
                <Outlet />
            </Card>
        </div>
    )
}
