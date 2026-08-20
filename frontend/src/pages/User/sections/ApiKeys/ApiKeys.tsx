import { Trans, useLingui } from '@lingui/react/macro'
import { useState } from 'react'

import type { CreatedApiKey } from '@/api/apiKeys'
import { useApiKeys } from '@/api/service/apiKeys'
import { Button } from '@/design-system/Button'
import { PlusIcon } from '@/design-system/Icon'
import { ContentLayout } from '@/layouts/ContentLayout'

import { ApiKeysTable } from '../../components/ApiKeysTable'
import { CreateApiKeyDialog } from '../../components/CreateApiKeyDialog'
import { NewApiKeyBanner } from '../../components/NewApiKeyBanner'

export function ApiKeys() {
    const { t } = useLingui()
    const { data, error, isLoading, mutate } = useApiKeys()
    const [isCreateOpen, setIsCreateOpen] = useState(false)
    const [createdKey, setCreatedKey] = useState<CreatedApiKey | null>(null)

    async function handleCreated(apiKey: CreatedApiKey) {
        setCreatedKey(apiKey)
        setIsCreateOpen(false)
        await mutate()
    }

    return (
        <ContentLayout
            title={t`API keys`}
            description={t`Keys authenticate machine access to the API. The secret is shown once.`}
            actions={
                <Button onClick={() => setIsCreateOpen(true)}>
                    <PlusIcon />
                    <Trans>New key</Trans>
                </Button>
            }
        >
            {createdKey ? (
                <NewApiKeyBanner
                    apiKey={createdKey}
                    onDismiss={() => setCreatedKey(null)}
                />
            ) : null}

            <ApiKeysTable
                apiKeys={data ?? []}
                isLoading={isLoading}
                error={error}
                onRevoked={() => void mutate()}
            />

            <CreateApiKeyDialog
                isOpen={isCreateOpen}
                onOpenChange={setIsCreateOpen}
                onCreated={handleCreated}
            />
        </ContentLayout>
    )
}
