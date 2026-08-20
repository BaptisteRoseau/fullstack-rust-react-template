import { useLingui } from '@lingui/react/macro'
import { Outlet } from 'react-router'

import { Head } from '@/components/head/Head'

import { UserNav } from './components/UserNav'
import styles from './user.module.scss'

export function User() {
    const { t } = useLingui()

    return (
        <div className={styles.page}>
            <Head title={t`Your account`} />
            <UserNav />
            <div className={styles.content}>
                <Outlet />
            </div>
        </div>
    )
}
