import { Trans, useLingui } from '@lingui/react/macro'
import * as z from 'zod'

import { API_KEY_PERMISSIONS, type CreatedApiKey } from '@/api/domains/apiKeys'
import { useApiErrorMessage } from '@/api/errors'
import { useApiCreateApiKey } from '@/api/hooks/useApiCreateApiKey'
import { CheckboxGroupField } from '@/components/forms/fields/CheckboxGroupField'
import { TextField } from '@/components/forms/fields/TextField'
import { Form } from '@/components/forms/Form'
import { Button } from '@/design-system/Button'
import {
    DialogClose,
    DialogContent,
    DialogFooter,
    DialogRoot,
} from '@/design-system/Dialog'
import { useNotifications } from '@/stores/notifications'

import styles from './create-api-key-dialog.module.scss'

const createApiKeySchema = z.object({
    name: z.string().min(1),
    permissions: z.array(z.enum(API_KEY_PERMISSIONS)).min(1),
})

export type CreateApiKeyDialogProps = {
    isOpen: boolean
    onOpenChange: (isOpen: boolean) => void
    onCreated: (apiKey: CreatedApiKey) => void
}

export function CreateApiKeyDialog({
    isOpen,
    onOpenChange,
    onCreated,
}: CreateApiKeyDialogProps) {
    const { t } = useLingui()
    const { trigger, isMutating } = useApiCreateApiKey()
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    async function handleSubmit(values: z.infer<typeof createApiKeySchema>) {
        try {
            onCreated(await trigger(values))
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not create the API key`,
                message: apiErrorMessage(error),
            })
        }
    }

    return (
        <DialogRoot open={isOpen} onOpenChange={onOpenChange}>
            <DialogContent
                title={t`New API key`}
                description={t`The secret is displayed once, right after creation.`}
            >
                <Form
                    schema={createApiKeySchema}
                    onSubmit={handleSubmit}
                    className={styles.form}
                    defaultValues={{ name: '', permissions: ['read'] }}
                >
                    {() => (
                        <>
                            <TextField
                                name="name"
                                label={t`Name`}
                                placeholder={t`CI deploy key`}
                            />
                            <CheckboxGroupField
                                name="permissions"
                                label={t`Permissions`}
                                options={API_KEY_PERMISSIONS.map(
                                    (permission) => ({
                                        value: permission,
                                        label: permission,
                                    }),
                                )}
                            />
                            <DialogFooter>
                                <DialogClose asChild>
                                    <Button variant="secondary">
                                        <Trans>Cancel</Trans>
                                    </Button>
                                </DialogClose>
                                <Button type="submit" disabled={isMutating}>
                                    <Trans>Create key</Trans>
                                </Button>
                            </DialogFooter>
                        </>
                    )}
                </Form>
            </DialogContent>
        </DialogRoot>
    )
}
