import { Trans } from '@lingui/react/macro'
import { Outlet } from 'react-router'

import { AppFooter } from '@/components/layout/AppFooter'
import { AppHeader } from '@/components/layout/AppHeader'

import styles from './app-layout.module.scss'

const MAIN_CONTENT_ID = 'main'

export function AppLayout() {
    return (
        <div className={styles.layout}>
            <a className={styles.skipLink} href={`#${MAIN_CONTENT_ID}`}>
                <Trans>Skip to main content</Trans>
            </a>
            <AppHeader />
            <main id={MAIN_CONTENT_ID} className={styles.content} tabIndex={-1}>
                <Outlet />
            </main>
            <AppFooter />
        </div>
    )
}
