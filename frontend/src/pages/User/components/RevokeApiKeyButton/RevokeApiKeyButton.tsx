import { useLingui } from '@lingui/react/macro'

import type { ApiKey } from '@/api/apiKeys'
import { apiErrorMessage } from '@/api/errors'
import { useRevokeApiKey } from '@/api/service/apiKeys'
import { ConfirmationDialog } from '@/components/ConfirmationDialog'
import { IconButton } from '@/design-system/Button'
import { TrashIcon } from '@/design-system/Icon'
import { useNotifications } from '@/stores/notifications'

export type RevokeApiKeyButtonProps = {
    apiKey: ApiKey
    onRevoked: () => void
}

export function RevokeApiKeyButton({
    apiKey,
    onRevoked,
}: RevokeApiKeyButtonProps) {
    const { t } = useLingui()
    const { trigger, isMutating } = useRevokeApiKey(apiKey.id)
    const addNotification = useNotifications((state) => state.addNotification)

    async function handleConfirm() {
        try {
            await trigger()
            addNotification({ type: 'success', title: t`API key revoked` })
            onRevoked()
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not revoke the API key`,
                message: apiErrorMessage(error, t`Unexpected error`),
            })
        }
    }

    return (
        <ConfirmationDialog
            title={t`Revoke ${apiKey.name}`}
            description={t`Applications using this key will stop working immediately.`}
            confirmLabel={t`Revoke`}
            isConfirming={isMutating}
            onConfirm={() => void handleConfirm()}
            trigger={
                <IconButton
                    aria-label={t`Revoke ${apiKey.name}`}
                    variant="ghost"
                    size="sm"
                >
                    <TrashIcon />
                </IconButton>
            }
        />
    )
}
