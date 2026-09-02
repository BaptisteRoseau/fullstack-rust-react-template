import { useLingui } from '@lingui/react/macro'
import { Link } from 'react-router'

import type { DriveDirectory } from '@/api/domains/drive'
import { Card } from '@/design-system/Card'
import { FolderIcon } from '@/design-system/Icon'
import { driveDirectoryPath } from '@/router/constants'
import { formatDate } from '@/utils/date'

import type { DriveTrailEntry } from '../../types'
import { EntryActions } from '../EntryActions'

import styles from './directory-card.module.scss'

export type DirectoryCardProps = {
    directory: DriveDirectory
    /** The trail leading here, extended with this folder when it is opened. */
    trail: DriveTrailEntry[]
    siblings: DriveDirectory[]
}

export function DirectoryCard({
    directory,
    trail,
    siblings,
}: DirectoryCardProps) {
    const { t } = useLingui()

    return (
        <Card className={styles.card}>
            <Link
                to={driveDirectoryPath(directory.id)}
                state={{
                    trail: [
                        ...trail,
                        { id: directory.id, name: directory.name },
                    ],
                }}
                className={styles.link}
                aria-label={t`Open ${directory.name}`}
            >
                <span className={styles.thumbnail}>
                    <FolderIcon size={40} />
                </span>
                <span className={styles.name}>{directory.name}</span>
                <span className={styles.meta}>
                    {formatDate(directory.updatedAt)}
                </span>
            </Link>
            <div className={styles.actions}>
                <EntryActions
                    kind="directory"
                    entryId={directory.id}
                    name={directory.name}
                    destinations={siblings}
                />
            </div>
        </Card>
    )
}
