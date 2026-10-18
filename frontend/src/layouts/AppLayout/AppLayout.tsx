import { Outlet } from 'react-router'

import { AppFooter } from '@/components/layout/AppFooter'
import { AppHeader } from '@/components/layout/AppHeader'
import { SkipLink } from '@/components/layout/SkipLink'

import styles from './app-layout.module.scss'

const MAIN_CONTENT_ID = 'main'

export function AppLayout() {
    return (
        <div className={styles.layout}>
            <SkipLink targetId={MAIN_CONTENT_ID} />
            <AppHeader />
            <main id={MAIN_CONTENT_ID} className={styles.content} tabIndex={-1}>
                <Outlet />
            </main>
            <AppFooter />
        </div>
    )
}
