import { Trans, useLingui } from '@lingui/react/macro'
import { Link } from 'react-router'

import { Head } from '@/components/head/Head'
import { Button } from '@/design-system/Button'
import { PATHS } from '@/router/constants'

import styles from './not-found.module.scss'

export function NotFound() {
    const { t } = useLingui()

    return (
        <div className={styles.wrapper}>
            <Head title={t`Page not found`} />
            <h1>
                <Trans>Page not found</Trans>
            </h1>
            <p className={styles.description}>
                <Trans>The page you are looking for does not exist.</Trans>
            </p>
            <Button asChild>
                <Link to={PATHS.home}>
                    <Trans>Back to home</Trans>
                </Link>
            </Button>
        </div>
    )
}
