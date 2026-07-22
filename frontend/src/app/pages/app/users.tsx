import { t } from '@lingui/core/macro'
import { Trans } from '@lingui/react/macro'
import { QueryClient } from '@tanstack/react-query'

import { ContentLayout } from '@/components/layouts'
import { getUsersQueryOptions } from '@/features/users/api/get-users'
import { UsersList } from '@/features/users/components/users-list'
import { Authorization, ROLES } from '@/lib/authorization'

export const clientLoader = (queryClient: QueryClient) => async () => {
    const query = getUsersQueryOptions()

    return (
        queryClient.getQueryData(query.queryKey) ??
        (await queryClient.fetchQuery(query))
    )
}

const UsersRoute = () => {
    return (
        <ContentLayout title={t`Users`}>
            <Authorization
                forbiddenFallback={
                    <div>
                        <Trans>Only admin can view this.</Trans>
                    </div>
                }
                allowedRoles={[ROLES.ADMIN]}
            >
                <UsersList />
            </Authorization>
        </ContentLayout>
    )
}

export default UsersRoute
