import { Slot } from '@radix-ui/react-slot'
import clsx from 'clsx'

import styles from './link.module.scss'

export type LinkProps = React.AnchorHTMLAttributes<HTMLAnchorElement> & {
    variant?: 'default' | 'muted'
    asChild?: boolean
}

export function Link({
    variant = 'default',
    asChild = false,
    className,
    ...props
}: LinkProps) {
    const Component = asChild ? Slot : 'a'

    return (
        <Component
            className={clsx(styles.link, styles[variant], className)}
            {...props}
        />
    )
}
