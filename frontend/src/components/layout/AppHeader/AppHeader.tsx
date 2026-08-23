import { Trans, useLingui } from '@lingui/react/macro'
import { Link } from 'react-router'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { Logo } from '@/components/layout/Logo'
import { ThemeSwitcher } from '@/components/layout/ThemeSwitcher'
import { UserMenu } from '@/components/layout/UserMenu'
import { Button } from '@/design-system/Button'
import { Spinner } from '@/design-system/Spinner'
import { PATHS } from '@/router/constants'

import styles from './app-header.module.scss'

export function AppHeader() {
    const { t } = useLingui()
    const { data: user, isLoading } = useApiCurrentUser()

    return (
        <header className={styles.header}>
            <div className={styles.inner}>
                <Logo label={t`Fullstack Template`} />
                <div className={styles.actions}>
                    <ThemeSwitcher />
                    <nav className={styles.account} aria-label={t`Account`}>
                        {isLoading ? (
                            <Spinner size="sm" label={t`Loading`} />
                        ) : null}
                        {!isLoading && user ? <UserMenu user={user} /> : null}
                        {!isLoading && !user ? (
                            <>
                                <Button variant="ghost" asChild>
                                    <Link to={PATHS.login}>
                                        <Trans>Log in</Trans>
                                    </Link>
                                </Button>
                                <Button asChild>
                                    <Link to={PATHS.register}>
                                        <Trans>Register</Trans>
                                    </Link>
                                </Button>
                            </>
                        ) : null}
                    </nav>
                </div>
            </div>
        </header>
    )
}
