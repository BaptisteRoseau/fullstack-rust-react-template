import { Trans, useLingui } from '@lingui/react/macro'

import { useApiCurrentUser } from '@/api/hooks/useApiCurrentUser'
import { Button } from '@/design-system/Button'
import { Card } from '@/design-system/Card'
import { CheckIcon, CopyIcon } from '@/design-system/Icon'
import { useCopyToClipboard } from '@/hooks/useCopyToClipboard'

import styles from './your-user-id.module.scss'

/**
 * Sharing takes a raw user id because this backend has no user directory to
 * search. Showing the signed-in user their own id is what makes the exchange
 * possible at all: it is the value the other side has to be given.
 */
export function YourUserId() {
    const { t } = useLingui()
    const { data: user } = useApiCurrentUser()
    const { isCopied, copy } = useCopyToClipboard()

    if (!user) {
        return null
    }

    return (
        <Card className={styles.card}>
            <div className={styles.text}>
                <p className={styles.title}>
                    <Trans>Your user ID</Trans>
                </p>
                <p className={styles.description}>
                    <Trans>
                        Give this to someone so they can share a folder with
                        you.
                    </Trans>
                </p>
            </div>
            <code className={styles.id}>{user.id}</code>
            <Button
                variant="secondary"
                size="sm"
                aria-label={t`Copy your user ID`}
                onClick={() => void copy(user.id)}
            >
                {isCopied ? <CheckIcon /> : <CopyIcon />}
                {isCopied ? <Trans>Copied</Trans> : <Trans>Copy</Trans>}
            </Button>
        </Card>
    )
}
