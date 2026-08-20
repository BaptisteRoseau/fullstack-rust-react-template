import { Trans, useLingui } from '@lingui/react/macro'
import clsx from 'clsx'
import { NavLink } from 'react-router'

import { KeyIcon, UserIcon } from '@/design-system/Icon'
import { PATHS } from '@/router/constants'

import styles from './user-nav.module.scss'

export function UserNav() {
    const { t } = useLingui()

    const linkClassName = ({ isActive }: { isActive: boolean }) =>
        clsx(styles.link, isActive && styles.active)

    return (
        <nav className={styles.nav} aria-label={t`Account sections`}>
            <NavLink to={PATHS.user.information} end className={linkClassName}>
                <UserIcon />
                <Trans>Information</Trans>
            </NavLink>
            <NavLink to={PATHS.user.apiKeys} className={linkClassName}>
                <KeyIcon />
                <Trans>API keys</Trans>
            </NavLink>
        </nav>
    )
}
