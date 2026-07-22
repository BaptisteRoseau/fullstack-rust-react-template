import { t } from '@lingui/core/macro'
import { Trans } from '@lingui/react/macro'

import { ContentLayout } from '@/components/layouts'
import { useUser } from '@/lib/auth'
import { ROLES } from '@/lib/authorization'

const DashboardRoute = () => {
    const user = useUser()
    return (
        <ContentLayout title={t`Dashboard`}>
            <h1 className="text-xl">
                <Trans>
                    Welcome{' '}
                    <b>{`${user.data?.firstName} ${user.data?.lastName}`}</b>
                </Trans>
            </h1>
            <h4 className="my-3">
                <Trans>
                    Your role is : <b>{user.data?.role}</b>
                </Trans>
            </h4>
            <p className="font-medium">
                <Trans>In this application you can:</Trans>
            </p>
            {user.data?.role === ROLES.USER && (
                <ul className="my-4 list-inside list-disc">
                    <li>
                        <Trans>Create comments in discussions</Trans>
                    </li>
                    <li>
                        <Trans>Delete own comments</Trans>
                    </li>
                </ul>
            )}
            {user.data?.role === ROLES.ADMIN && (
                <ul className="my-4 list-inside list-disc">
                    <li>
                        <Trans>Create discussions</Trans>
                    </li>
                    <li>
                        <Trans>Edit discussions</Trans>
                    </li>
                    <li>
                        <Trans>Delete discussions</Trans>
                    </li>
                    <li>
                        <Trans>Comment on discussions</Trans>
                    </li>
                    <li>
                        <Trans>Delete all comments</Trans>
                    </li>
                </ul>
            )}
        </ContentLayout>
    )
}

export default DashboardRoute
