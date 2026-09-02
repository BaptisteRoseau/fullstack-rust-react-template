import { Trans, useLingui } from '@lingui/react/macro'
import * as z from 'zod'

import { useApiErrorMessage } from '@/api/errors'
import { useApiCreateDirectory } from '@/api/hooks/useApiCreateDirectory'
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

import styles from './create-directory-dialog.module.scss'

const createDirectorySchema = z.object({
    name: z
        .string()
        .min(1)
        .max(255)
        .refine((name) => !name.includes('/')),
})

export type CreateDirectoryDialogProps = {
    parentId: string | null
    isOpen: boolean
    onOpenChange: (isOpen: boolean) => void
}

export function CreateDirectoryDialog({
    parentId,
    isOpen,
    onOpenChange,
}: CreateDirectoryDialogProps) {
    const { t } = useLingui()
    const { trigger, isMutating } = useApiCreateDirectory()
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    async function handleSubmit(values: z.infer<typeof createDirectorySchema>) {
        try {
            await trigger({ name: values.name, parentId })
            addNotification({
                type: 'success',
                title: t`Folder created`,
                message: values.name,
            })
            onOpenChange(false)
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not create the folder`,
                message: apiErrorMessage(error),
            })
        }
    }

    return (
        <DialogRoot open={isOpen} onOpenChange={onOpenChange}>
            <DialogContent
                title={t`New folder`}
                description={t`The folder is created where you are now.`}
            >
                <Form
                    schema={createDirectorySchema}
                    onSubmit={handleSubmit}
                    className={styles.form}
                    defaultValues={{ name: '' }}
                >
                    {() => (
                        <>
                            <TextField
                                name="name"
                                label={t`Name`}
                                placeholder={t`Invoices`}
                            />
                            <DialogFooter>
                                <DialogClose asChild>
                                    <Button variant="secondary">
                                        <Trans>Cancel</Trans>
                                    </Button>
                                </DialogClose>
                                <Button type="submit" disabled={isMutating}>
                                    <Trans>Create folder</Trans>
                                </Button>
                            </DialogFooter>
                        </>
                    )}
                </Form>
            </DialogContent>
        </DialogRoot>
    )
}
