import { Trans, useLingui } from '@lingui/react/macro'
import { Link } from 'react-router'

import { useCurrentUser } from '@/api/service/auth'
import { Logo } from '@/components/layout/Logo'
import { UserMenu } from '@/components/layout/UserMenu'
import { Button } from '@/design-system/Button'
import { Spinner } from '@/design-system/Spinner'
import { PATHS } from '@/router/constants'

import styles from './app-header.module.scss'

export function AppHeader() {
    const { t } = useLingui()
    const { data: user, isLoading } = useCurrentUser()

    return (
        <header className={styles.header}>
            <div className={styles.inner}>
                <Logo label={t`Fullstack Template`} />
                <nav className={styles.actions} aria-label={t`Account`}>
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
        </header>
    )
}
