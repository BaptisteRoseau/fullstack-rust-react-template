import { Trans } from '@lingui/react/macro'

import styles from './skip-link.module.scss'

export type SkipLinkProps = {
    targetId: string
}

export function SkipLink({ targetId }: SkipLinkProps) {
    return (
        <a className={styles.skipLink} href={`#${targetId}`}>
            <Trans>Skip to main content</Trans>
        </a>
    )
}
