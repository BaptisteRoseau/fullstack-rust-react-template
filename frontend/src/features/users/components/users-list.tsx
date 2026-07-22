import { t } from '@lingui/core/macro'

import { Spinner } from '@/components/ui/spinner'
import { Table } from '@/components/ui/table'
import { formatDate } from '@/utils/format'

import { useUsers } from '../api/get-users'

import { DeleteUser } from './delete-user'

export const UsersList = () => {
    const usersQuery = useUsers()

    if (usersQuery.isLoading) {
        return (
            <div className="flex h-48 w-full items-center justify-center">
                <Spinner size="lg" />
            </div>
        )
    }

    const users = usersQuery.data?.data

    if (!users) return null

    return (
        <Table
            data={users}
            columns={[
                {
                    title: t`First Name`,
                    field: 'firstName',
                },
                {
                    title: t`Last Name`,
                    field: 'lastName',
                },
                {
                    title: t`Email`,
                    field: 'email',
                },
                {
                    title: t`Role`,
                    field: 'role',
                },
                {
                    title: t`Created At`,
                    field: 'createdAt',
                    Cell({ entry: { createdAt } }) {
                        return <span>{formatDate(createdAt)}</span>
                    },
                },
                {
                    title: '',
                    field: 'id',
                    Cell({ entry: { id } }) {
                        return <DeleteUser id={id} />
                    },
                },
            ]}
        />
    )
}
