import type { FallbackProps } from 'react-error-boundary'

import { Button } from '@/design-system/Button'

import styles from './error-fallback.module.scss'

export function ErrorFallback({ error }: FallbackProps) {
    return (
        <div className={styles.fallback} role="alert">
            <h1>Something went wrong</h1>
            <p className={styles.message}>{(error as Error).message}</p>
            <Button onClick={() => window.location.assign('/')}>
                Back to home
            </Button>
        </div>
    )
}
