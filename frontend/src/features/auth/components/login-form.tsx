import { Trans } from '@lingui/react/macro'
import { Link, useSearchParams } from 'react-router'

import { paths } from '@/config/paths'
import { loginUrl } from '@/lib/auth'

export const LoginForm = () => {
    const [searchParams] = useSearchParams()
    const redirectTo = searchParams.get('redirectTo')

    return (
        <div>
            <p className="mb-4 text-center text-sm text-gray-600">
                <Trans>
                    You will be redirected to our secure sign-in page.
                </Trans>
            </p>
            <a
                href={loginUrl(redirectTo)}
                className="flex w-full justify-center rounded-md bg-blue-600 px-4 py-2 font-medium text-white hover:bg-blue-500"
            >
                <Trans>Continue to sign in</Trans>
            </a>
            <div className="mt-2 flex items-center justify-end">
                <div className="text-sm">
                    <Link
                        to={paths.auth.register.getHref(redirectTo)}
                        className="font-medium text-blue-600 hover:text-blue-500"
                    >
                        <Trans>Register</Trans>
                    </Link>
                </div>
            </div>
        </div>
    )
}
