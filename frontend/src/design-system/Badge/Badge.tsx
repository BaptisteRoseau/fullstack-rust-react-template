import clsx from 'clsx'

import styles from './badge.module.scss'

export type BadgeProps = React.HTMLAttributes<HTMLSpanElement> & {
    variant?: 'neutral' | 'success' | 'warning' | 'danger'
}

export function Badge({
    variant = 'neutral',
    className,
    ...props
}: BadgeProps) {
    return (
        <span
            className={clsx(styles.badge, styles[variant], className)}
            {...props}
        />
    )
}
