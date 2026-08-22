import { Trans, useLingui } from '@lingui/react/macro'
import { useSWRConfig } from 'swr'
import * as z from 'zod'

import { ME_ENDPOINT } from '@/api/auth'
import { apiErrorMessage } from '@/api/errors'
import { useCurrentUser, useUpdateProfile } from '@/api/service/auth'
import { TextField } from '@/components/forms/fields/TextField'
import { Form } from '@/components/forms/Form'
import { Badge } from '@/design-system/Badge'
import { Button } from '@/design-system/Button'
import { Card } from '@/design-system/Card'
import { Spinner } from '@/design-system/Spinner'
import { ContentLayout } from '@/layouts/ContentLayout'
import { useNotifications } from '@/stores/notifications'
import { formatDateTime } from '@/utils/date'

import styles from './information.module.scss'

const profileSchema = z.object({
    firstName: z.string().min(1),
    lastName: z.string().min(1),
})

export function Information() {
    const { t } = useLingui()
    const { data: user, isLoading } = useCurrentUser()
    const { trigger, isMutating } = useUpdateProfile()
    const { mutate } = useSWRConfig()
    const addNotification = useNotifications((state) => state.addNotification)

    if (isLoading) {
        return <Spinner size="lg" label={t`Loading`} />
    }

    if (!user) {
        return null
    }

    async function handleSubmit(values: z.infer<typeof profileSchema>) {
        try {
            await trigger(values)
            await mutate(ME_ENDPOINT)
            addNotification({
                type: 'success',
                title: t`Profile updated`,
            })
        } catch (error) {
            addNotification({
                type: 'error',
                title: t`Could not update the profile`,
                message: apiErrorMessage(error, t`Unexpected error`),
            })
        }
    }

    return (
        <ContentLayout
            title={t`Information`}
            description={t`The profile shared with the rest of the application.`}
        >
            <Card>
                <dl className={styles.facts}>
                    <div className={styles.fact}>
                        <dt>
                            <Trans>Email</Trans>
                        </dt>
                        <dd>{user.email}</dd>
                    </div>
                    <div className={styles.fact}>
                        <dt>
                            <Trans>Role</Trans>
                        </dt>
                        <dd>
                            <Badge>{user.role}</Badge>
                        </dd>
                    </div>
                    <div className={styles.fact}>
                        <dt>
                            <Trans>Member since</Trans>
                        </dt>
                        <dd>{formatDateTime(user.createdAt)}</dd>
                    </div>
                </dl>
            </Card>

            <Card>
                <Form
                    schema={profileSchema}
                    onSubmit={handleSubmit}
                    className={styles.form}
                    defaultValues={{
                        firstName: user.firstName,
                        lastName: user.lastName,
                    }}
                >
                    {() => (
                        <>
                            <TextField name="firstName" label={t`First name`} />
                            <TextField name="lastName" label={t`Last name`} />
                            <div className={styles.actions}>
                                <Button type="submit" disabled={isMutating}>
                                    <Trans>Save changes</Trans>
                                </Button>
                            </div>
                        </>
                    )}
                </Form>
            </Card>
        </ContentLayout>
    )
}
