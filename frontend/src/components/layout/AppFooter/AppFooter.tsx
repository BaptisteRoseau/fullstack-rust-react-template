import { Trans, useLingui } from '@lingui/react/macro'
import { Link as RouterLink } from 'react-router'

import { LocaleSwitcher } from '@/components/layout/LocaleSwitcher'
import { Link } from '@/design-system/Link'
import { PATHS } from '@/router/constants'

import styles from './app-footer.module.scss'

export function AppFooter() {
    const { t } = useLingui()

    return (
        <footer className={styles.footer}>
            <div className={styles.inner}>
                <p className={styles.copyright}>
                    <Trans>© {new Date().getFullYear()} Driftbox</Trans>
                </p>
                <nav className={styles.links} aria-label={t`Footer`}>
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
                    <Link
                        variant="muted"
                        href="https://github.com/BaptisteRoseau/fullstack-rust-react-template"
                    >
                        <Trans>GitHub</Trans>
                    </Link>
                    <Link variant="muted" href="/api/swagger">
                        <Trans>API reference</Trans>
                    </Link>
                </nav>
                <LocaleSwitcher />
            </div>
        </footer>
    )
}
