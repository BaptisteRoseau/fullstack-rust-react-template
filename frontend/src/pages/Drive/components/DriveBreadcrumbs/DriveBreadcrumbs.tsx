import { Trans, useLingui } from '@lingui/react/macro'
import { Link } from 'react-router'

import { ChevronRightIcon } from '@/design-system/Icon'
import { driveDirectoryPath, PATHS } from '@/router/constants'

import type { DriveTrailEntry } from '../../types'

import styles from './drive-breadcrumbs.module.scss'

export type DriveBreadcrumbsProps = {
    trail: DriveTrailEntry[]
    /** `false` when the ancestors above the current directory are unknown. */
    isComplete: boolean
}

export function DriveBreadcrumbs({ trail, isComplete }: DriveBreadcrumbsProps) {
    const { t } = useLingui()

    return (
        <nav className={styles.breadcrumbs} aria-label={t`Breadcrumb`}>
            <ol className={styles.list}>
                <li className={styles.item}>
                    <Link to={PATHS.drive.root} className={styles.link}>
                        <Trans>Home</Trans>
                    </Link>
                </li>
                {isComplete ? null : (
                    <li className={styles.item} aria-hidden>
                        <ChevronRightIcon className={styles.separator} />
                        <span className={styles.elision}>…</span>
                    </li>
                )}
                {trail.map((entry, index) => (
                    <li key={entry.id} className={styles.item}>
                        <ChevronRightIcon className={styles.separator} />
                        {index === trail.length - 1 ? (
                            <span
                                aria-current="page"
                                className={styles.current}
                            >
                                {entry.name}
                            </span>
                        ) : (
                            <Link
                                to={driveDirectoryPath(entry.id)}
                                state={{ trail: trail.slice(0, index + 1) }}
                                className={styles.link}
                            >
                                {entry.name}
                            </Link>
                        )}
                    </li>
                ))}
            </ol>
        </nav>
    )
}
