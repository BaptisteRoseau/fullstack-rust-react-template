import { Trans, useLingui } from '@lingui/react/macro'
import * as z from 'zod'

import { useApiErrorMessage } from '@/api/errors'
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

import { useDriveEntryUpdate } from '../../hooks/useDriveEntryUpdate'
import type { DriveEntryKind } from '../../types'

import styles from './rename-entry-dialog.module.scss'

const renameSchema = z.object({
    name: z
        .string()
        .min(1)
        .max(255)
        .refine((name) => !name.includes('/')),
})

export type RenameEntryDialogProps = {
    kind: DriveEntryKind
    entryId: string
    name: string
    isOpen: boolean
    onOpenChange: (isOpen: boolean) => void
}

export function RenameEntryDialog({
    kind,
    entryId,
    name,
    isOpen,
    onOpenChange,
}: RenameEntryDialogProps) {
    const { t } = useLingui()
    const { trigger, isMutating } = useDriveEntryUpdate(kind, entryId)
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    async function handleSubmit(values: z.infer<typeof renameSchema>) {
        try {
            await trigger({ name: values.name })
            addNotification({ type: 'success', title: t`Renamed` })
            onOpenChange(false)
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not rename ${name}`,
                message: apiErrorMessage(error),
            })
        }
    }

    return (
        <DialogRoot open={isOpen} onOpenChange={onOpenChange}>
            <DialogContent
                title={t`Rename ${name}`}
                description={t`Only the name changes; the contents stay where they are.`}
            >
                <Form
                    schema={renameSchema}
                    onSubmit={handleSubmit}
                    className={styles.form}
                    defaultValues={{ name }}
                >
                    {() => (
                        <>
                            <TextField name="name" label={t`Name`} />
                            <DialogFooter>
                                <DialogClose asChild>
                                    <Button variant="secondary">
                                        <Trans>Cancel</Trans>
                                    </Button>
                                </DialogClose>
                                <Button type="submit" disabled={isMutating}>
                                    <Trans>Rename</Trans>
                                </Button>
                            </DialogFooter>
                        </>
                    )}
                </Form>
            </DialogContent>
        </DialogRoot>
    )
}
