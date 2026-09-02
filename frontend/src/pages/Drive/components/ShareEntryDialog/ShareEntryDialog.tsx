import { Trans, useLingui } from '@lingui/react/macro'
import * as z from 'zod'

import { PERMISSION_LEVELS } from '@/api/domains/drive'
import { useApiErrorMessage } from '@/api/errors'
import { SelectField } from '@/components/forms/fields/SelectField'
import { TextField } from '@/components/forms/fields/TextField'
import { Form } from '@/components/forms/Form'
import { Button, IconButton } from '@/design-system/Button'
import { Card } from '@/design-system/Card'
import { DialogContent, DialogFooter, DialogRoot } from '@/design-system/Dialog'
import { TrashIcon } from '@/design-system/Icon'
import { Spinner } from '@/design-system/Spinner'
import { useNotifications } from '@/stores/notifications'

import { useDriveEntrySharing } from '../../hooks/useDriveEntrySharing'
import type { DriveEntryKind } from '../../types'

import styles from './share-entry-dialog.module.scss'

/**
 * The grantee is typed in as a raw id: this backend exposes no directory of
 * users to search, so there is nothing to autocomplete against.
 */
const shareSchema = z.object({
    userId: z.string().uuid(),
    level: z.enum(PERMISSION_LEVELS),
})

export type ShareEntryDialogProps = {
    kind: DriveEntryKind
    entryId: string
    name: string
    isOpen: boolean
    onOpenChange: (isOpen: boolean) => void
}

export function ShareEntryDialog({
    kind,
    entryId,
    name,
    isOpen,
    onOpenChange,
}: ShareEntryDialogProps) {
    const { t } = useLingui()
    const { grants, grant, revoke } = useDriveEntrySharing(
        kind,
        entryId,
        isOpen,
    )
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    const levelOptions = [
        { value: 'viewer', label: t`Viewer` },
        { value: 'editor', label: t`Editor` },
        { value: 'manager', label: t`Manager` },
    ]

    async function handleGrant(values: z.infer<typeof shareSchema>) {
        try {
            await grant.trigger(values)
            addNotification({ type: 'success', title: t`${name} shared` })
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not share ${name}`,
                message: apiErrorMessage(error),
            })
        }
    }

    async function handleRevoke(userId: string) {
        try {
            await revoke.trigger(userId)
            addNotification({ type: 'success', title: t`Access revoked` })
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not revoke the access`,
                message: apiErrorMessage(error),
            })
        }
    }

    return (
        <DialogRoot open={isOpen} onOpenChange={onOpenChange}>
            <DialogContent
                title={t`Share ${name}`}
                description={t`Everyone you add here sees this entry, and everything under it.`}
            >
                <div className={styles.grants}>
                    {grants.isLoading ? (
                        <Card className={styles.state}>
                            <Spinner label={t`Loading`} />
                        </Card>
                    ) : null}
                    {!grants.isLoading && grants.error ? (
                        <Card className={styles.state} role="alert">
                            <Trans>
                                The people with access could not be loaded.
                            </Trans>
                        </Card>
                    ) : null}
                    {!grants.isLoading &&
                    !grants.error &&
                    grants.data?.length === 0 ? (
                        <Card className={styles.state}>
                            <Trans>Nobody else has access yet.</Trans>
                        </Card>
                    ) : null}
                    {grants.data?.map((permission) => (
                        <div key={permission.id} className={styles.grant}>
                            <span className={styles.grantee}>
                                {permission.grantee}
                            </span>
                            <span className={styles.level}>
                                {permission.level}
                            </span>
                            <IconButton
                                aria-label={t`Revoke the access of ${permission.grantee}`}
                                variant="ghost"
                                size="sm"
                                disabled={revoke.isMutating}
                                onClick={() =>
                                    void handleRevoke(permission.grantee)
                                }
                            >
                                <TrashIcon />
                            </IconButton>
                        </div>
                    ))}
                </div>

                <Form
                    schema={shareSchema}
                    onSubmit={handleGrant}
                    className={styles.form}
                    defaultValues={{ userId: '', level: 'viewer' }}
                >
                    {() => (
                        <>
                            <TextField
                                name="userId"
                                label={t`User ID`}
                                description={t`Ask them for their user ID — there is no directory search yet.`}
                                placeholder={t`00000000-0000-0000-0000-000000000000`}
                            />
                            <SelectField
                                name="level"
                                label={t`Access level`}
                                options={levelOptions}
                            />
                            <DialogFooter>
                                <Button
                                    type="submit"
                                    disabled={grant.isMutating}
                                >
                                    <Trans>Share</Trans>
                                </Button>
                            </DialogFooter>
                        </>
                    )}
                </Form>
            </DialogContent>
        </DialogRoot>
    )
}
