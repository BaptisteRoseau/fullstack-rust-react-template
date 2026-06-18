import { Trans } from '@lingui/macro'
import { Outlet } from 'react-router'

import { DashboardLayout } from '@/components/layouts'

export const ErrorBoundary = () => {
    return (
        <div>
            <Trans>Something went wrong!</Trans>
        </div>
    )
}

const AppRoot = () => {
    return (
        <DashboardLayout>
            <Outlet />
        </DashboardLayout>
    )
}

export default AppRoot
