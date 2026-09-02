import { Trans } from '@lingui/react/macro'

import { Card } from '@/design-system/Card'
import {
    KeyIcon,
    MonitorIcon,
    ShieldIcon,
    UserIcon,
} from '@/design-system/Icon'

import styles from './feature-grid.module.scss'

export function FeatureGrid() {
    return (
        <section className={styles.grid}>
            <Card className={styles.feature}>
                <ShieldIcon size={24} className={styles.icon} />
                <h2>
                    <Trans>Compressed and encrypted</Trans>
                </h2>
                <p className={styles.description}>
                    <Trans>
                        Every upload is squeezed down and sealed before it is
                        stored. You never configure a cipher or pick an archive
                        format.
                    </Trans>
                </p>
            </Card>
            <Card className={styles.feature}>
                <MonitorIcon size={24} className={styles.icon} />
                <h2>
                    <Trans>Instant previews</Trans>
                </h2>
                <p className={styles.description}>
                    <Trans>
                        Thumbnails for photos and first-page previews for
                        documents appear as soon as a file lands, without
                        downloading it.
                    </Trans>
                </p>
            </Card>
            <Card className={styles.feature}>
                <KeyIcon size={24} className={styles.icon} />
                <h2>
                    <Trans>Sharing you can aim</Trans>
                </h2>
                <p className={styles.description}>
                    <Trans>
                        Share exactly the folder or file you choose, nothing
                        more, as viewer, editor or manager.
                    </Trans>
                </p>
            </Card>
            <Card className={styles.feature}>
                <UserIcon size={24} className={styles.icon} />
                <h2>
                    <Trans>Built for several people</Trans>
                </h2>
                <p className={styles.description}>
                    <Trans>
                        Each member signs in with their own account and sees
                        only their own files and what has been shared with them.
                    </Trans>
                </p>
            </Card>
        </section>
    )
}
