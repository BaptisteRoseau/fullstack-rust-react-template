import clsx from 'clsx'

import styles from './spinner.module.scss'

export type SpinnerProps = {
    size?: 'sm' | 'md' | 'lg'
    label: string
    className?: string
}

export function Spinner({ size = 'md', label, className }: SpinnerProps) {
    return (
        <span
            role="status"
            aria-label={label}
            className={clsx(styles.spinner, styles[size], className)}
        />
    )
}
