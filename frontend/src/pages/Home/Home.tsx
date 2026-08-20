import { Trans, useLingui } from '@lingui/react/macro'
import { Link } from 'react-router'

import { useCurrentUser } from '@/api/service/auth'
import { Head } from '@/components/head/Head'
import { Button } from '@/design-system/Button'
import { PATHS } from '@/router/constants'

import { FeatureGrid } from './components/FeatureGrid'
import styles from './home.module.scss'

export function Home() {
    const { t } = useLingui()
    const { data: user } = useCurrentUser()

    return (
        <>
            <Head
                title={t`Fullstack Template`}
                description={t`A Rust and React fullstack application template.`}
            />
            <section className={styles.hero}>
                <h1 className={styles.title}>
                    <Trans>Ship a fullstack app, not a toolchain</Trans>
                </h1>
                <p className={styles.subtitle}>
                    <Trans>
                        A Rust backend and a React frontend wired together with
                        authentication, API keys and observability from the
                        first commit.
                    </Trans>
                </p>
                <div className={styles.actions}>
                    {user ? (
                        <Button size="lg" asChild>
                            <Link to={PATHS.user.information}>
                                <Trans>Go to your account</Trans>
                            </Link>
                        </Button>
                    ) : (
                        <>
                            <Button size="lg" asChild>
                                <Link to={PATHS.register}>
                                    <Trans>Get started</Trans>
                                </Link>
                            </Button>
                            <Button size="lg" variant="secondary" asChild>
                                <Link to={PATHS.login}>
                                    <Trans>Log in</Trans>
                                </Link>
                            </Button>
                        </>
                    )}
                </div>
            </section>
            <FeatureGrid />
        </>
    )
}
