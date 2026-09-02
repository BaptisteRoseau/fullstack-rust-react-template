import { Trans, useLingui } from '@lingui/react/macro'
import { Link as RouterLink } from 'react-router'

import { Link } from '@/design-system/Link'
import { PATHS } from '@/router/constants'

import styles from './trust-strip.module.scss'

export function TrustStrip() {
    const { t } = useLingui()

    return (
        <section className={styles.strip}>
            <h2 className={styles.title}>
                <Trans>Sealed before it is stored</Trans>
            </h2>
            <p className={styles.description}>
                <Trans>
                    Uploads are compressed and encrypted while they are still in
                    memory, so nothing ever reaches the storage layer in the
                    clear. Permissions are checked on every read, and a share
                    grants access to what you picked and to nothing above it.
                </Trans>
            </p>
            <p className={styles.description}>
                <Trans>
                    Driftbox is a proof-of-concept build, so read the terms
                    before you trust it with anything you cannot afford to lose.
                </Trans>
            </p>
            <nav className={styles.links} aria-label={t`Legal`}>
                <Link variant="muted" asChild>
                    <RouterLink to={PATHS.legal.terms}>
                        <Trans>Terms of Service</Trans>
                    </RouterLink>
                </Link>
                <Link variant="muted" asChild>
                    <RouterLink to={PATHS.legal.mentions}>
                        <Trans>Legal mentions</Trans>
                    </RouterLink>
                </Link>
            </nav>
        </section>
    )
}
