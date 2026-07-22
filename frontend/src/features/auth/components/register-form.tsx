import { Trans } from '@lingui/react/macro'
import { Link, useSearchParams } from 'react-router'

import { paths } from '@/config/paths'
import { registerUrl } from '@/lib/auth'

export const RegisterForm = () => {
    const [searchParams] = useSearchParams()
    const redirectTo = searchParams.get('redirectTo')

    return (
        <div>
            <p className="mb-4 text-center text-sm text-gray-600">
                <Trans>
                    You will be redirected to our secure sign-up page to create
                    your account.
                </Trans>
            </p>
            <a
                href={registerUrl(redirectTo)}
                className="flex w-full justify-center rounded-md bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-500"
            >
                <Trans>Create an account</Trans>
            </a>
            <div className="mt-2 flex items-center justify-end">
                <div className="text-sm">
                    <Link
                        to={paths.auth.login.getHref(redirectTo)}
                        className="font-medium text-blue-600 hover:text-blue-500"
                    >
                        <Trans>Log In</Trans>
                    </Link>
                </div>
            </div>
        </div>
    )
}
