import { useLingui } from '@lingui/react/macro'

import type { ApiKey } from '@/api/domains/apiKeys'
import { useApiErrorMessage } from '@/api/errors'
import { useApiRevokeApiKey } from '@/api/hooks/useApiRevokeApiKey'
import { ConfirmationDialog } from '@/components/ConfirmationDialog'
import { IconButton } from '@/design-system/Button'
import { TrashIcon } from '@/design-system/Icon'
import { useNotifications } from '@/stores/notifications'

export type RevokeApiKeyButtonProps = {
    apiKey: ApiKey
}

export function RevokeApiKeyButton({ apiKey }: RevokeApiKeyButtonProps) {
    const { t } = useLingui()
    const { trigger, isMutating } = useApiRevokeApiKey(apiKey.id)
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    async function handleConfirm() {
        try {
            await trigger()
            addNotification({ type: 'success', title: t`API key revoked` })
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not revoke the API key`,
                message: apiErrorMessage(error),
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
