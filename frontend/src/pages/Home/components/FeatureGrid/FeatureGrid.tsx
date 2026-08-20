import { Trans } from '@lingui/react/macro'

import { Card } from '@/design-system/Card'
import { BoltIcon, LayersIcon, ShieldIcon } from '@/design-system/Icon'

import styles from './feature-grid.module.scss'

export function FeatureGrid() {
    return (
        <section className={styles.grid}>
            <Card className={styles.feature}>
                <ShieldIcon size={24} className={styles.icon} />
                <h2>
                    <Trans>Authentication built in</Trans>
                </h2>
                <p className={styles.description}>
                    <Trans>
                        OpenID Connect through the backend, with httpOnly
                        cookies and silent token refresh.
                    </Trans>
                </p>
            </Card>
            <Card className={styles.feature}>
                <BoltIcon size={24} className={styles.icon} />
                <h2>
                    <Trans>A typed API layer</Trans>
                </h2>
                <p className={styles.description}>
                    <Trans>
                        Endpoint declarations separated from the SWR hooks that
                        call them, mocked end to end.
                    </Trans>
                </p>
            </Card>
            <Card className={styles.feature}>
                <LayersIcon size={24} className={styles.icon} />
                <h2>
                    <Trans>A real design system</Trans>
                </h2>
                <p className={styles.description}>
                    <Trans>
                        Domain-agnostic primitives on SCSS Modules and design
                        tokens, every one of them in Storybook.
                    </Trans>
                </p>
            </Card>
        </section>
    )
}
