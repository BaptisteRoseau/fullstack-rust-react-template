import { t, Trans } from '@lingui/macro'
import { Trash } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { ConfirmationDialog } from '@/components/ui/dialog'
import { useNotifications } from '@/components/ui/notifications'

import { useDeleteComment } from '../api/delete-comment'

type DeleteCommentProps = {
    id: string
    discussionId: string
}

export const DeleteComment = ({ id, discussionId }: DeleteCommentProps) => {
    const { addNotification } = useNotifications()
    const deleteCommentMutation = useDeleteComment({
        discussionId,
        mutationConfig: {
            onSuccess: () => {
                addNotification({
                    type: 'success',
                    title: t`Comment Deleted`,
                })
            },
        },
    })

    return (
        <ConfirmationDialog
            isDone={deleteCommentMutation.isSuccess}
            icon="danger"
            title={t`Delete Comment`}
            body={t`Are you sure you want to delete this comment?`}
            triggerButton={
                <Button
                    variant="destructive"
                    size="sm"
                    icon={<Trash className="size-4" />}
                >
                    <Trans>Delete Comment</Trans>
                </Button>
            }
            confirmButton={
                <Button
                    isLoading={deleteCommentMutation.isPending}
                    type="button"
                    variant="destructive"
                    onClick={() =>
                        deleteCommentMutation.mutate({ commentId: id })
                    }
                >
                    <Trans>Delete Comment</Trans>
                </Button>
            }
        />
    )
}
