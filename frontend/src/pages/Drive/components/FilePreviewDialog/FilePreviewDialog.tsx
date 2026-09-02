import { Trans, useLingui } from '@lingui/react/macro'
import { useEffect, useState } from 'react'

import type { DriveFile } from '@/api/domains/drive'
import { useApiDownloadFile } from '@/api/hooks/useApiDownloadFile'
import { useApiFileContent } from '@/api/hooks/useApiFileContent'
import { Button } from '@/design-system/Button'
import { Card } from '@/design-system/Card'
import { DialogContent, DialogFooter, DialogRoot } from '@/design-system/Dialog'
import { DownloadIcon } from '@/design-system/Icon'
import { Spinner } from '@/design-system/Spinner'
import { useObjectUrl } from '@/hooks/useObjectUrl'
import { formatDateTime } from '@/utils/date'
import { formatFileSize, isPdf, mimeTypeGroup } from '@/utils/files'

import styles from './file-preview-dialog.module.scss'

/** Enough of a text file to judge it, without pinning megabytes in the DOM. */
const TEXT_PREVIEW_LIMIT = 50_000

export type FilePreviewDialogProps = {
    file: DriveFile
    isOpen: boolean
    onOpenChange: (isOpen: boolean) => void
}

/**
 * The decoded head of a text file. Like {@link useObjectUrl}, the blob is kept
 * next to what was read from it, so a render between a change and the effect
 * never shows the previous file's contents.
 */
function useTextPreview(blob: Blob | undefined, isText: boolean) {
    const [current, setCurrent] = useState<{ blob: Blob; text: string } | null>(
        null,
    )

    useEffect(() => {
        if (!blob || !isText) {
            return
        }

        let isCurrent = true
        void blob
            .slice(0, TEXT_PREVIEW_LIMIT)
            .text()
            .then((text) => {
                if (isCurrent) {
                    setCurrent({ blob, text })
                }
            })

        return () => {
            isCurrent = false
        }
    }, [blob, isText])

    if (!isText || !current || current.blob !== blob) {
        return null
    }
    return current.text
}

export function FilePreviewDialog({
    file,
    isOpen,
    onOpenChange,
}: FilePreviewDialogProps) {
    const { t } = useLingui()
    const isText = mimeTypeGroup(file.mimeType) === 'text'
    const isImage = mimeTypeGroup(file.mimeType) === 'image'
    const isDocument = isPdf(file.mimeType)
    const isRenderable = isText || isImage || isDocument

    const { data, error, isLoading } = useApiFileContent(
        isOpen && isRenderable ? file.id : undefined,
    )
    const url = useObjectUrl(isImage || isDocument ? data : null)
    const text = useTextPreview(data, isText)
    const { download, isDownloading } = useApiDownloadFile()

    return (
        <DialogRoot open={isOpen} onOpenChange={onOpenChange}>
            <DialogContent
                title={file.name}
                description={t`${formatFileSize(file.sizeBytes)} · ${file.mimeType} · added ${formatDateTime(file.createdAt)}`}
                className={styles.dialog}
            >
                <div className={styles.preview}>
                    {isRenderable && isLoading ? (
                        <Spinner label={t`Loading`} />
                    ) : null}

                    {isRenderable && error ? (
                        <Card className={styles.state} role="alert">
                            <Trans>The preview could not be loaded.</Trans>
                        </Card>
                    ) : null}

                    {isImage && url ? (
                        <img
                            src={url}
                            alt={file.name}
                            className={styles.image}
                        />
                    ) : null}

                    {isDocument && url ? (
                        <iframe
                            src={url}
                            title={file.name}
                            className={styles.document}
                        />
                    ) : null}

                    {isText && text !== null ? (
                        <>
                            <pre className={styles.text}>{text}</pre>
                            {file.sizeBytes > TEXT_PREVIEW_LIMIT ? (
                                <p className={styles.truncated}>
                                    <Trans>
                                        Only the first part of this file is
                                        shown. Download it to read the rest.
                                    </Trans>
                                </p>
                            ) : null}
                        </>
                    ) : null}

                    {isRenderable ? null : (
                        <Card className={styles.state}>
                            <Trans>
                                This kind of file cannot be previewed here.
                            </Trans>
                        </Card>
                    )}
                </div>

                <DialogFooter>
                    <Button
                        disabled={isDownloading}
                        onClick={() => void download(file.id, file.name)}
                    >
                        <DownloadIcon />
                        <Trans>Download</Trans>
                    </Button>
                </DialogFooter>
            </DialogContent>
        </DialogRoot>
    )
}
