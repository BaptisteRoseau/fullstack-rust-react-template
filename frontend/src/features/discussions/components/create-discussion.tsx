import { t } from '@lingui/core/macro'
import { Trans } from '@lingui/react/macro'
import { Plus } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Form, FormDrawer, Input, Textarea } from '@/components/ui/form'
import { useNotifications } from '@/components/ui/notifications'
import { Authorization, ROLES } from '@/lib/authorization'

import {
    createDiscussionInputSchema,
    useCreateDiscussion,
} from '../api/create-discussion'

export const CreateDiscussion = () => {
    const { addNotification } = useNotifications()
    const createDiscussionMutation = useCreateDiscussion({
        mutationConfig: {
            onSuccess: () => {
                addNotification({
                    type: 'success',
                    title: t`Discussion Created`,
                })
            },
        },
    })

    return (
        <Authorization allowedRoles={[ROLES.ADMIN]}>
            <FormDrawer
                isDone={createDiscussionMutation.isSuccess}
                triggerButton={
                    <Button size="sm" icon={<Plus className="size-4" />}>
                        <Trans>Create Discussion</Trans>
                    </Button>
                }
                title={t`Create Discussion`}
                submitButton={
                    <Button
                        form="create-discussion"
                        type="submit"
                        size="sm"
                        isLoading={createDiscussionMutation.isPending}
                    >
                        <Trans>Submit</Trans>
                    </Button>
                }
            >
                <Form
                    id="create-discussion"
                    onSubmit={(values) => {
                        createDiscussionMutation.mutate({ data: values })
                    }}
                    schema={createDiscussionInputSchema}
                >
                    {({ register, formState }) => (
                        <>
                            <Input
                                label={t`Title`}
                                error={formState.errors['title']}
                                registration={register('title')}
                            />

                            <Textarea
                                label={t`Body`}
                                error={formState.errors['body']}
                                registration={register('body')}
                            />
                        </>
                    )}
                </Form>
            </FormDrawer>
        </Authorization>
    )
}
