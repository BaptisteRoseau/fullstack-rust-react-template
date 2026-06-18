import { t, Trans } from '@lingui/macro'

import { Button } from '@/components/ui/button'
import { ConfirmationDialog } from '@/components/ui/dialog'
import { useNotifications } from '@/components/ui/notifications'
import { useUser } from '@/lib/auth'

import { useDeleteUser } from '../api/delete-user'

type DeleteUserProps = {
    id: string
}

export const DeleteUser = ({ id }: DeleteUserProps) => {
    const user = useUser()
    const { addNotification } = useNotifications()
    const deleteUserMutation = useDeleteUser({
        mutationConfig: {
            onSuccess: () => {
                addNotification({
                    type: 'success',
                    title: t`User Deleted`,
                })
            },
        },
    })

    if (user.data?.id === id) return null

    return (
        <ConfirmationDialog
            icon="danger"
            title={t`Delete User`}
            body={t`Are you sure you want to delete this user?`}
            triggerButton={
                <Button variant="destructive">
                    <Trans>Delete</Trans>
                </Button>
            }
            confirmButton={
                <Button
                    isLoading={deleteUserMutation.isPending}
                    type="button"
                    variant="destructive"
                    onClick={() => deleteUserMutation.mutate({ userId: id })}
                >
                    <Trans>Delete User</Trans>
                </Button>
            }
        />
    )
}
