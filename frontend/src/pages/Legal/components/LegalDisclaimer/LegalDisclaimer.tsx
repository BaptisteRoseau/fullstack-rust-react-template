import { Trans } from '@lingui/react/macro'

import { Badge } from '@/design-system/Badge'

import styles from './legal-disclaimer.module.scss'

export function LegalDisclaimer() {
    return (
        <aside className={styles.disclaimer}>
            <Badge variant="warning">
                <Trans>Demo content</Trans>
            </Badge>
            <p className={styles.text}>
                <Trans>
                    Placeholder for this proof of concept, not a real legal
                    agreement. Driftbox is a fictional product, the company and
                    addresses below do not exist, and none of this text has been
                    written or reviewed by a lawyer.
                </Trans>
            </p>
        </aside>
    )
}
