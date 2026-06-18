import { t, Trans } from '@lingui/macro'
import { Trash } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { ConfirmationDialog } from '@/components/ui/dialog'
import { useNotifications } from '@/components/ui/notifications'
import { Authorization, ROLES } from '@/lib/authorization'

import { useDeleteDiscussion } from '../api/delete-discussion'

type DeleteDiscussionProps = {
    id: string
}

export const DeleteDiscussion = ({ id }: DeleteDiscussionProps) => {
    const { addNotification } = useNotifications()
    const deleteDiscussionMutation = useDeleteDiscussion({
        mutationConfig: {
            onSuccess: () => {
                addNotification({
                    type: 'success',
                    title: t`Discussion Deleted`,
                })
            },
        },
    })

    return (
        <Authorization allowedRoles={[ROLES.ADMIN]}>
            <ConfirmationDialog
                icon="danger"
                title={t`Delete Discussion`}
                body={t`Are you sure you want to delete this discussion?`}
                triggerButton={
                    <Button
                        variant="destructive"
                        icon={<Trash className="size-4" />}
                    >
                        <Trans>Delete Discussion</Trans>
                    </Button>
                }
                confirmButton={
                    <Button
                        isLoading={deleteDiscussionMutation.isPending}
                        type="button"
                        variant="destructive"
                        onClick={() =>
                            deleteDiscussionMutation.mutate({
                                discussionId: id,
                            })
                        }
                    >
                        <Trans>Delete Discussion</Trans>
                    </Button>
                }
            />
        </Authorization>
    )
}
