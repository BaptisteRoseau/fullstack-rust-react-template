import { Trans, useLingui } from '@lingui/react/macro'
import { Link, useSearchParams } from 'react-router'

import { authRedirectUrl } from '@/api/auth'
import { Head } from '@/components/head/Head'
import { Button } from '@/design-system/Button'
import { Link as TextLink } from '@/design-system/Link'
import { PATHS } from '@/router/constants'

import styles from './login.module.scss'

export function Login() {
    const { t } = useLingui()
    const [searchParams] = useSearchParams()
    const redirectTo = searchParams.get('redirect') ?? PATHS.home

    return (
        <>
            <Head title={t`Log in`} />
            <h1 className={styles.title}>
                <Trans>Log in</Trans>
            </h1>
            <p className={styles.description}>
                <Trans>
                    You will be redirected to the identity provider to sign in.
                </Trans>
            </p>
            <Button
                size="lg"
                className={styles.action}
                onClick={() =>
                    window.location.assign(authRedirectUrl('login', redirectTo))
                }
            >
                <Trans>Continue to sign in</Trans>
            </Button>
            <p className={styles.footer}>
                <Trans>No account yet?</Trans>{' '}
                <TextLink asChild>
                    <Link to={PATHS.register}>
                        <Trans>Register</Trans>
                    </Link>
                </TextLink>
            </p>
        </>
    )
}
