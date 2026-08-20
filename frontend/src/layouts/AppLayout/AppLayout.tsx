import { Outlet } from 'react-router'

import { AppFooter } from '@/components/layout/AppFooter'
import { AppHeader } from '@/components/layout/AppHeader'

import styles from './app-layout.module.scss'

export function AppLayout() {
    return (
        <div className={styles.layout}>
            <AppHeader />
            <main className={styles.content}>
                <Outlet />
            </main>
            <AppFooter />
        </div>
    )
}
