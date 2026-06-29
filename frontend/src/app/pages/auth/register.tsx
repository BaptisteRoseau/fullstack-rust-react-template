import { t } from '@lingui/macro'

import { AuthLayout } from '@/components/layouts/auth-layout'
import { RegisterForm } from '@/features/auth/components/register-form'

const RegisterRoute = () => {
    return (
        <AuthLayout title={t`Register your account`}>
            <RegisterForm />
        </AuthLayout>
    )
}

export default RegisterRoute
