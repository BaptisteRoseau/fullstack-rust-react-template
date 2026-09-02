import { useLingui } from '@lingui/react/macro'

import type { DriveDirectory, DriveFile } from '@/api/domains/drive'
import { useApiThumbnail } from '@/api/hooks/useApiThumbnail'
import { Badge } from '@/design-system/Badge'
import { Card } from '@/design-system/Card'
import { useBooleanState } from '@/hooks/useBooleanState'
import { formatFileSize, savedPercentage } from '@/utils/files'

import { EntryActions } from '../EntryActions'
import { FilePreviewDialog } from '../FilePreviewDialog'
import { FileTypeIcon } from '../FileTypeIcon'

import styles from './file-card.module.scss'

export type FileCardProps = {
    file: DriveFile
    /** The folders in view, offered as move destinations. */
    destinations: DriveDirectory[]
}

export function FileCard({ file, destinations }: FileCardProps) {
    const { t } = useLingui()
    const preview = useBooleanState()
    const { url } = useApiThumbnail(file.hasThumbnail ? file.id : undefined)
    const saved = savedPercentage(file.sizeBytes, file.storedSizeBytes)

    return (
        <Card className={styles.card}>
            <button
                type="button"
                className={styles.opener}
                onClick={preview.setTrue}
                aria-label={t`Preview ${file.name}`}
            >
                <span className={styles.thumbnail}>
                    {url ? (
                        <img src={url} alt="" className={styles.image} />
                    ) : (
                        <FileTypeIcon mimeType={file.mimeType} size={40} />
                    )}
                </span>
                <span className={styles.name}>{file.name}</span>
                <span className={styles.meta}>
                    {formatFileSize(file.sizeBytes)}
                    {saved === null ? null : (
                        <Badge
                            variant="success"
                            title={t`Stored as ${formatFileSize(file.storedSizeBytes)} after compression and encryption`}
                        >
                            {t`−${saved}%`}
                        </Badge>
                    )}
                </span>
            </button>
            <div className={styles.actions}>
                <EntryActions
                    kind="file"
                    entryId={file.id}
                    name={file.name}
                    file={file}
                    destinations={destinations}
                    onPreview={preview.setTrue}
                />
            </div>

            <FilePreviewDialog
                file={file}
                isOpen={preview.value}
                onOpenChange={(isOpen) =>
                    isOpen ? preview.setTrue() : preview.setFalse()
                }
            />
        </Card>
    )
}
