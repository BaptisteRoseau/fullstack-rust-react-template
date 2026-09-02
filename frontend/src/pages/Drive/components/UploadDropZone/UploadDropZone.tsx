import { Trans, useLingui } from '@lingui/react/macro'
import clsx from 'clsx'
import { useRef, useState } from 'react'

import { useApiErrorMessage } from '@/api/errors'
import { useApiUploadFile } from '@/api/hooks/useApiUploadFile'
import { Button } from '@/design-system/Button'
import { UploadIcon } from '@/design-system/Icon'
import { Spinner } from '@/design-system/Spinner'
import { useNotifications } from '@/stores/notifications'

import styles from './upload-drop-zone.module.scss'

export type UploadDropZoneProps = {
    parentId: string | null
    children: React.ReactNode
}

/**
 * Wraps the listing so a file can be dropped anywhere on it, and exposes the
 * same upload through a real `<input type="file">` — the picker is the path
 * that has to work, the drop is the shortcut.
 */
export function UploadDropZone({ parentId, children }: UploadDropZoneProps) {
    const { t } = useLingui()
    const inputRef = useRef<HTMLInputElement>(null)
    const [isDraggedOver, setIsDraggedOver] = useState(false)
    const { trigger, isMutating } = useApiUploadFile()
    const apiErrorMessage = useApiErrorMessage()
    const addNotification = useNotifications((state) => state.addNotification)

    async function upload(files: FileList | null) {
        for (const file of Array.from(files ?? [])) {
            try {
                await trigger({ file, parentId })
                addNotification({
                    type: 'success',
                    title: t`Upload complete`,
                    message: file.name,
                })
            } catch (error) {
                addNotification({
                    type: 'error',
                    title: t`Could not upload ${file.name}`,
                    message: apiErrorMessage(error),
                })
            }
        }
    }

    return (
        <div
            className={clsx(styles.zone, isDraggedOver && styles.active)}
            onDragOver={(event) => {
                event.preventDefault()
                setIsDraggedOver(true)
            }}
            onDragLeave={() => setIsDraggedOver(false)}
            onDrop={(event) => {
                event.preventDefault()
                setIsDraggedOver(false)
                void upload(event.dataTransfer.files)
            }}
        >
            <div className={styles.bar}>
                <Button
                    variant="secondary"
                    disabled={isMutating}
                    onClick={() => inputRef.current?.click()}
                >
                    <UploadIcon />
                    <Trans>Upload files</Trans>
                </Button>
                <p className={styles.hint}>
                    <Trans>…or drop them anywhere below.</Trans>
                </p>
                {isMutating ? <Spinner size="sm" label={t`Uploading`} /> : null}
            </div>

            <input
                ref={inputRef}
                type="file"
                multiple
                className={styles.input}
                aria-label={t`Choose files to upload`}
                onChange={(event) => {
                    void upload(event.target.files)
                    event.target.value = ''
                }}
            />

            {children}
        </div>
    )
}
