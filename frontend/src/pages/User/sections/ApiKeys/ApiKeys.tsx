import { Trans, useLingui } from '@lingui/react/macro'
import { useState } from 'react'

import type { CreatedApiKey } from '@/api/domains/apiKeys'
import { useApiApiKeys } from '@/api/hooks/useApiApiKeys'
import { Button } from '@/design-system/Button'
import { PlusIcon } from '@/design-system/Icon'
import { ContentLayout } from '@/layouts/ContentLayout'

import { ApiKeysTable } from '../../components/ApiKeysTable'
import { CreateApiKeyDialog } from '../../components/CreateApiKeyDialog'
import { NewApiKeyBanner } from '../../components/NewApiKeyBanner'

export function ApiKeys() {
    const { t } = useLingui()
    const { data, error, isLoading } = useApiApiKeys()
    const [isCreateOpen, setIsCreateOpen] = useState(false)
    const [createdKey, setCreatedKey] = useState<CreatedApiKey | null>(null)

    function handleCreated(apiKey: CreatedApiKey) {
        setCreatedKey(apiKey)
        setIsCreateOpen(false)
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
            />

            <CreateApiKeyDialog
                isOpen={isCreateOpen}
                onOpenChange={setIsCreateOpen}
                onCreated={handleCreated}
            />
        </ContentLayout>
    )
}
