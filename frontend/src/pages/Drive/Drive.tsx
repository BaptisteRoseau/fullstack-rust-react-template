import { Trans, useLingui } from '@lingui/react/macro'
import { useParams } from 'react-router'

import { useApiEntries } from '@/api/hooks/useApiEntries'
import { Head } from '@/components/head/Head'
import { Button } from '@/design-system/Button'
import { PlusIcon } from '@/design-system/Icon'
import { useBooleanState } from '@/hooks/useBooleanState'
import { ContentLayout } from '@/layouts/ContentLayout'

import { CreateDirectoryDialog } from './components/CreateDirectoryDialog'
import { DriveBreadcrumbs } from './components/DriveBreadcrumbs'
import { DriveListing } from './components/DriveListing'
import { UploadDropZone } from './components/UploadDropZone'
import { YourUserId } from './components/YourUserId'
import styles from './drive.module.scss'
import { useDriveTrail } from './hooks/useDriveTrail'

export function Drive() {
    const { t } = useLingui()
    const { directoryId } = useParams()
    const { data, error, isLoading } = useApiEntries(directoryId)
    const trail = useDriveTrail(directoryId, data?.directory)
    const createDirectory = useBooleanState()

    return (
        <div className={styles.page}>
            <Head title={t`Drive`} />
            <ContentLayout
                title={t`Drive`}
                description={t`Your files. Compressed and encrypted before they are stored, and shareable with anyone you give access to.`}
                actions={
                    <Button onClick={createDirectory.setTrue}>
                        <PlusIcon />
                        <Trans>New folder</Trans>
                    </Button>
                }
            >
                <DriveBreadcrumbs
                    trail={trail.entries}
                    isComplete={trail.isComplete}
                />

                <UploadDropZone parentId={directoryId ?? null}>
                    <DriveListing
                        entries={data}
                        trail={trail.entries}
                        isLoading={isLoading}
                        error={error}
                    />
                </UploadDropZone>

                <YourUserId />

                <CreateDirectoryDialog
                    parentId={directoryId ?? null}
                    isOpen={createDirectory.value}
                    onOpenChange={(isOpen) =>
                        isOpen
                            ? createDirectory.setTrue()
                            : createDirectory.setFalse()
                    }
                />
            </ContentLayout>
        </div>
    )
}
