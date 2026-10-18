import { useLingui } from '@lingui/react/macro'
import { Outlet } from 'react-router'

import { Logo } from '@/components/layout/Logo'
import { SkipLink } from '@/components/layout/SkipLink'
import { Card } from '@/design-system/Card'

import styles from './auth-layout.module.scss'

const MAIN_CONTENT_ID = 'main'

export function AuthLayout() {
    const { t } = useLingui()

    return (
        <div className={styles.layout}>
            <SkipLink targetId={MAIN_CONTENT_ID} />
            <Logo label={t`Fullstack Template`} />
            <main id={MAIN_CONTENT_ID} className={styles.content} tabIndex={-1}>
                <Card className={styles.card}>
                    <Outlet />
                </Card>
            </main>
        </div>
    )
}
