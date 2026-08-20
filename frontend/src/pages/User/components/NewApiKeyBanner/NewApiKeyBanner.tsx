import { Trans, useLingui } from '@lingui/react/macro'

import type { CreatedApiKey } from '@/api/apiKeys'
import { Button, IconButton } from '@/design-system/Button'
import { CheckIcon, CloseIcon, CopyIcon } from '@/design-system/Icon'
import { useCopyToClipboard } from '@/hooks/useCopyToClipboard'

import styles from './new-api-key-banner.module.scss'

export type NewApiKeyBannerProps = {
    apiKey: CreatedApiKey
    onDismiss: () => void
}

export function NewApiKeyBanner({ apiKey, onDismiss }: NewApiKeyBannerProps) {
    const { t } = useLingui()
    const { isCopied, copy } = useCopyToClipboard()

    return (
        <div className={styles.banner} role="status">
            <div className={styles.header}>
                <p className={styles.title}>
                    <Trans>Copy {apiKey.name} now</Trans>
                </p>
                <IconButton
                    aria-label={t`Dismiss`}
                    variant="ghost"
                    size="sm"
                    onClick={onDismiss}
                >
                    <CloseIcon />
                </IconButton>
            </div>
            <p className={styles.description}>
                <Trans>This secret will not be shown again.</Trans>
            </p>
            <div className={styles.row}>
                <code className={styles.key}>{apiKey.key}</code>
                <Button
                    variant="secondary"
                    onClick={() => void copy(apiKey.key)}
                >
                    {isCopied ? <CheckIcon /> : <CopyIcon />}
                    {isCopied ? <Trans>Copied</Trans> : <Trans>Copy</Trans>}
                </Button>
            </div>
        </div>
    )
}
