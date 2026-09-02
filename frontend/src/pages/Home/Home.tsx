import { Trans, useLingui } from '@lingui/react/macro'
import { Link } from 'react-router'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { Head } from '@/components/head/Head'
import { Button } from '@/design-system/Button'
import { PATHS } from '@/router/constants'

import { FeatureGrid } from './components/FeatureGrid'
import { TrustStrip } from './components/TrustStrip'
import styles from './home.module.scss'

export function Home() {
    const { t } = useLingui()
    const { data: user } = useApiCurrentUser()

    return (
        <>
            <Head
                title={t`Driftbox`}
                description={t`Multi-user cloud storage that compresses and encrypts every file on upload.`}
            />
            <section className={styles.hero}>
                <h1 className={styles.title}>
                    <Trans>Cloud storage that secures itself</Trans>
                </h1>
                <p className={styles.subtitle}>
                    <Trans>
                        Driftbox keeps the documents, photos and archives of
                        your whole team in one place. Every file is compressed
                        and encrypted the moment it is uploaded — no setting to
                        turn on, no key to remember.
                    </Trans>
                </p>
                <div className={styles.actions}>
                    {user ? (
                        <Button size="lg" asChild>
                            <Link to={PATHS.drive.root}>
                                <Trans>Open your drive</Trans>
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
            <TrustStrip />
        </>
    )
}
