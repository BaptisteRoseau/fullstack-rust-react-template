import { Trans, useLingui } from '@lingui/react/macro'
import * as z from 'zod'

import type { DriveDirectory } from '@/api/domains/drive'
import { useApiErrorMessage } from '@/api/errors'
import { SelectField } from '@/components/forms/fields/SelectField'
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

import styles from './move-entry-dialog.module.scss'

const ROOT_VALUE = 'root'

const moveSchema = z.object({ destination: z.string().min(1) })

export type MoveEntryDialogProps = {
    kind: DriveEntryKind
    entryId: string
    name: string
    /** The folders offered as a destination: the ones in view, plus the root. */
    destinations: DriveDirectory[]
    isOpen: boolean
    onOpenChange: (isOpen: boolean) => void
}

export function MoveEntryDialog({
    kind,
    entryId,
    name,
    destinations,
    isOpen,
    onOpenChange,
}: MoveEntryDialogProps) {
    const { t } = useLingui()
    const { trigger, isMutating } = useDriveEntryUpdate(kind, entryId)
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    const options = [
        { value: ROOT_VALUE, label: t`Home` },
        ...destinations
            .filter((directory) => directory.id !== entryId)
            .map((directory) => ({
                value: directory.id,
                label: directory.name,
            })),
    ]

    async function handleSubmit(values: z.infer<typeof moveSchema>) {
        try {
            await trigger({
                parentId:
                    values.destination === ROOT_VALUE
                        ? null
                        : values.destination,
            })
            addNotification({ type: 'success', title: t`${name} moved` })
            onOpenChange(false)
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not move ${name}`,
                message: apiErrorMessage(error),
            })
        }
    }

    return (
        <DialogRoot open={isOpen} onOpenChange={onOpenChange}>
            <DialogContent
                title={t`Move ${name}`}
                description={t`Pick a folder from the one you are in, or send it back to Home.`}
            >
                <Form
                    schema={moveSchema}
                    onSubmit={handleSubmit}
                    className={styles.form}
                    defaultValues={{ destination: ROOT_VALUE }}
                >
                    {() => (
                        <>
                            <SelectField
                                name="destination"
                                label={t`Destination`}
                                options={options}
                            />
                            <DialogFooter>
                                <DialogClose asChild>
                                    <Button variant="secondary">
                                        <Trans>Cancel</Trans>
                                    </Button>
                                </DialogClose>
                                <Button type="submit" disabled={isMutating}>
                                    <Trans>Move</Trans>
                                </Button>
                            </DialogFooter>
                        </>
                    )}
                </Form>
            </DialogContent>
        </DialogRoot>
    )
}
