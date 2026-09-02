import { Trans, useLingui } from '@lingui/react/macro'

import type { DriveDirectory, DriveFile } from '@/api/domains/drive'
import { useApiErrorMessage } from '@/api/errors'
import { useApiDownloadFile } from '@/api/hooks/useApiDownloadFile'
import { ConfirmationDialog } from '@/components/ConfirmationDialog'
import { IconButton } from '@/design-system/Button'
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownSeparator,
    DropdownTrigger,
} from '@/design-system/Dropdown'
import {
    DownloadIcon,
    EyeIcon,
    LayersIcon,
    MoveIcon,
    PencilIcon,
    ShareIcon,
    TrashIcon,
} from '@/design-system/Icon'
import { useBooleanState } from '@/hooks/useBooleanState'
import { useNotifications } from '@/stores/notifications'

import { useDriveEntryDelete } from '../../hooks/useDriveEntryDelete'
import type { DriveEntryKind } from '../../types'
import { MoveEntryDialog } from '../MoveEntryDialog'
import { RenameEntryDialog } from '../RenameEntryDialog'
import { ShareEntryDialog } from '../ShareEntryDialog'

import styles from './entry-actions.module.scss'

export type EntryActionsProps = {
    kind: DriveEntryKind
    entryId: string
    name: string
    /** Present for a file: downloading needs its metadata. */
    file?: DriveFile
    /** The folders in view, offered as move destinations. */
    destinations: DriveDirectory[]
    /** Opens the preview the card itself owns; absent for a folder. */
    onPreview?: () => void
}

/**
 * Every per-entry command, and the dialogs they open. The dialogs are mounted
 * here rather than at the page so each card owns its own state; they render
 * nothing until opened, and `ShareEntryDialog` only fetches once it is. The
 * preview is the exception: the card body opens it too, so the card owns it and
 * this menu only asks for it.
 */
export function EntryActions({
    kind,
    entryId,
    name,
    file,
    destinations,
    onPreview,
}: EntryActionsProps) {
    const { t } = useLingui()
    const rename = useBooleanState()
    const move = useBooleanState()
    const share = useBooleanState()
    const { trigger: remove, isMutating } = useDriveEntryDelete(kind, entryId)
    const { download, isDownloading } = useApiDownloadFile()
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    async function handleDelete() {
        try {
            await remove()
            addNotification({ type: 'success', title: t`${name} deleted` })
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not delete ${name}`,
                message: apiErrorMessage(error),
            })
        }
    }

    async function handleDownload() {
        if (!file) {
            return
        }
        try {
            await download(file.id, file.name)
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not download ${name}`,
                message: apiErrorMessage(error),
            })
        }
    }

    return (
        <>
            <Dropdown>
                <DropdownTrigger asChild>
                    <IconButton
                        aria-label={t`Actions for ${name}`}
                        variant="ghost"
                        size="sm"
                        className={styles.trigger}
                    >
                        <LayersIcon />
                    </IconButton>
                </DropdownTrigger>
                <DropdownContent>
                    {file ? (
                        <>
                            <DropdownItem
                                disabled={!onPreview}
                                onSelect={() => onPreview?.()}
                            >
                                <EyeIcon />
                                <Trans>Preview</Trans>
                            </DropdownItem>
                            <DropdownItem
                                disabled={isDownloading}
                                onSelect={() => void handleDownload()}
                            >
                                <DownloadIcon />
                                <Trans>Download</Trans>
                            </DropdownItem>
                            <DropdownSeparator />
                        </>
                    ) : null}
                    <DropdownItem onSelect={rename.setTrue}>
                        <PencilIcon />
                        <Trans>Rename</Trans>
                    </DropdownItem>
                    <DropdownItem onSelect={move.setTrue}>
                        <MoveIcon />
                        <Trans>Move to…</Trans>
                    </DropdownItem>
                    <DropdownItem onSelect={share.setTrue}>
                        <ShareIcon />
                        <Trans>Share</Trans>
                    </DropdownItem>
                </DropdownContent>
            </Dropdown>

            <ConfirmationDialog
                title={t`Delete ${name}`}
                description={
                    kind === 'directory'
                        ? t`Everything inside this folder is deleted with it. This cannot be undone.`
                        : t`This file is deleted for everyone it was shared with. This cannot be undone.`
                }
                confirmLabel={t`Delete`}
                isConfirming={isMutating}
                onConfirm={() => void handleDelete()}
                trigger={
                    <IconButton
                        aria-label={t`Delete ${name}`}
                        variant="ghost"
                        size="sm"
                        className={styles.trigger}
                    >
                        <TrashIcon />
                    </IconButton>
                }
            />

            <RenameEntryDialog
                kind={kind}
                entryId={entryId}
                name={name}
                isOpen={rename.value}
                onOpenChange={(isOpen) =>
                    isOpen ? rename.setTrue() : rename.setFalse()
                }
            />

            <MoveEntryDialog
                kind={kind}
                entryId={entryId}
                name={name}
                destinations={destinations}
                isOpen={move.value}
                onOpenChange={(isOpen) =>
                    isOpen ? move.setTrue() : move.setFalse()
                }
            />

            <ShareEntryDialog
                kind={kind}
                entryId={entryId}
                name={name}
                isOpen={share.value}
                onOpenChange={(isOpen) =>
                    isOpen ? share.setTrue() : share.setFalse()
                }
            />
        </>
    )
}
