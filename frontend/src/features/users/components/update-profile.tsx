import { t, Trans } from '@lingui/macro'
import { Pen } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Form, FormDrawer, Input, Textarea } from '@/components/ui/form'
import { useNotifications } from '@/components/ui/notifications'
import { useUser } from '@/lib/auth'

import {
    updateProfileInputSchema,
    useUpdateProfile,
} from '../api/update-profile'

export const UpdateProfile = () => {
    const user = useUser()
    const { addNotification } = useNotifications()
    const updateProfileMutation = useUpdateProfile({
        mutationConfig: {
            onSuccess: () => {
                addNotification({
                    type: 'success',
                    title: t`Profile Updated`,
                })
            },
        },
    })

    return (
        <FormDrawer
            isDone={updateProfileMutation.isSuccess}
            triggerButton={
                <Button icon={<Pen className="size-4" />} size="sm">
                    <Trans>Update Profile</Trans>
                </Button>
            }
            title={t`Update Profile`}
            submitButton={
                <Button
                    form="update-profile"
                    type="submit"
                    size="sm"
                    isLoading={updateProfileMutation.isPending}
                >
                    <Trans>Submit</Trans>
                </Button>
            }
        >
            <Form
                id="update-profile"
                onSubmit={(values) => {
                    updateProfileMutation.mutate({ data: values })
                }}
                options={{
                    defaultValues: {
                        firstName: '',
                        lastName: '',
                        email: user.data?.email ?? '',
                        bio: '',
                    },
                }}
                schema={updateProfileInputSchema}
            >
                {({ register, formState }) => (
                    <>
                        <Input
                            label={t`First Name`}
                            error={formState.errors['firstName']}
                            registration={register('firstName')}
                        />
                        <Input
                            label={t`Last Name`}
                            error={formState.errors['lastName']}
                            registration={register('lastName')}
                        />
                        <Input
                            label={t`Email Address`}
                            type="email"
                            error={formState.errors['email']}
                            registration={register('email')}
                        />

                        <Textarea
                            label={t`Bio`}
                            error={formState.errors['bio']}
                            registration={register('bio')}
                        />
                    </>
                )}
            </Form>
        </FormDrawer>
    )
}
