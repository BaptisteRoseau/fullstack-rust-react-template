import { t } from '@lingui/core/macro'
import { Trans } from '@lingui/react/macro'

import { ContentLayout } from '@/components/layouts'
import { UpdateProfile } from '@/features/users/components/update-profile'
import { useUser } from '@/lib/auth'

type EntryProps = {
    label: string
    value: string
}
const Entry = ({ label, value }: EntryProps) => (
    <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
        <dt className="text-sm font-medium text-gray-500">{label}</dt>
        <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
            {value}
        </dd>
    </div>
)

const ProfileRoute = () => {
    const user = useUser()

    if (!user.data) return null

    return (
        <ContentLayout title={t`Profile`}>
            <div className="overflow-hidden bg-white shadow sm:rounded-lg">
                <div className="px-4 py-5 sm:px-6">
                    <div className="flex justify-between">
                        <h3 className="text-lg leading-6 font-medium text-gray-900">
                            <Trans>User Information</Trans>
                        </h3>
                        <UpdateProfile />
                    </div>
                    <p className="mt-1 max-w-2xl text-sm text-gray-500">
                        <Trans>Personal details of the user.</Trans>
                    </p>
                </div>
                <div className="border-t border-gray-200 px-4 py-5 sm:p-0">
                    <dl className="sm:divide-y sm:divide-gray-200">
                        <Entry
                            label={t`First Name`}
                            value={user.data.firstName}
                        />
                        <Entry
                            label={t`Last Name`}
                            value={user.data.lastName}
                        />
                        <Entry
                            label={t`Email Address`}
                            value={user.data.email}
                        />
                        <Entry label={t`Role`} value={user.data.role} />
                        <Entry label={t`Bio`} value={user.data.bio} />
                    </dl>
                </div>
            </div>
        </ContentLayout>
    )
}

export default ProfileRoute
