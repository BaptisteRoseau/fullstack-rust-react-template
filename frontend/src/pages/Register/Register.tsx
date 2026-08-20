import { Trans, useLingui } from '@lingui/react/macro'
import { Link, useSearchParams } from 'react-router'

import { authRedirectUrl } from '@/api/auth'
import { Head } from '@/components/head/Head'
import { Button } from '@/design-system/Button'
import { Link as TextLink } from '@/design-system/Link'
import { PATHS } from '@/router/constants'

import styles from './register.module.scss'

export function Register() {
    const { t } = useLingui()
    const [searchParams] = useSearchParams()
    const redirectTo = searchParams.get('redirect') ?? PATHS.user.information

    return (
        <>
            <Head title={t`Register`} />
            <h1 className={styles.title}>
                <Trans>Create your account</Trans>
            </h1>
            <p className={styles.description}>
                <Trans>
                    You will be redirected to the identity provider to create
                    your account.
                </Trans>
            </p>
            <Button
                size="lg"
                className={styles.action}
                onClick={() =>
                    window.location.assign(
                        authRedirectUrl('register', redirectTo),
                    )
                }
            >
                <Trans>Continue to registration</Trans>
            </Button>
            <p className={styles.footer}>
                <Trans>Already have an account?</Trans>{' '}
                <TextLink asChild>
                    <Link to={PATHS.login}>
                        <Trans>Log in</Trans>
                    </Link>
                </TextLink>
            </p>
        </>
    )
}
