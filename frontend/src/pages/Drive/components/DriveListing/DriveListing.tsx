import { Trans, useLingui } from '@lingui/react/macro'

import type { DriveEntries } from '@/api/domains/drive'
import { Card } from '@/design-system/Card'
import { Spinner } from '@/design-system/Spinner'

import type { DriveTrailEntry } from '../../types'
import { DirectoryCard } from '../DirectoryCard'
import { FileCard } from '../FileCard'

import styles from './drive-listing.module.scss'

export type DriveListingProps = {
    entries: DriveEntries | undefined
    trail: DriveTrailEntry[]
    isLoading: boolean
    error: unknown
}

/** Folders first, then files — both sorted by name, as a file manager does. */
const byName = (left: { name: string }, right: { name: string }) =>
    left.name.localeCompare(right.name)

export function DriveListing({
    entries,
    trail,
    isLoading,
    error,
}: DriveListingProps) {
    const { t } = useLingui()

    if (isLoading) {
        return (
            <Card className={styles.state}>
                <Spinner label={t`Loading`} />
            </Card>
        )
    }

    if (error) {
        return (
            <Card className={styles.state} role="alert">
                <Trans>This folder could not be loaded.</Trans>
            </Card>
        )
    }

    const directories = [...(entries?.directories ?? [])].sort(byName)
    const files = [...(entries?.files ?? [])].sort(byName)

    if (directories.length === 0 && files.length === 0) {
        return (
            <Card className={styles.state}>
                <Trans>This folder is empty. Upload a file to start.</Trans>
            </Card>
        )
    }

    return (
        <ul className={styles.grid}>
            {directories.map((directory) => (
                <li key={directory.id}>
                    <DirectoryCard
                        directory={directory}
                        trail={trail}
                        siblings={directories}
                    />
                </li>
            ))}
            {files.map((file) => (
                <li key={file.id}>
                    <FileCard file={file} destinations={directories} />
                </li>
            ))}
        </ul>
    )
}
