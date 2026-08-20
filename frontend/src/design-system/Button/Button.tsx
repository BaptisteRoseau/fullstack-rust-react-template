import { Slot } from '@radix-ui/react-slot'
import clsx from 'clsx'

import styles from './button.module.scss'

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
    size?: 'sm' | 'md' | 'lg'
    asChild?: boolean
}

export function Button({
    variant = 'primary',
    size = 'md',
    asChild = false,
    className,
    type = 'button',
    ...props
}: ButtonProps) {
    const Component = asChild ? Slot : 'button'

    return (
        <Component
            className={clsx(
                styles.button,
                styles[variant],
                styles[size],
                className,
            )}
            type={asChild ? undefined : type}
            {...props}
        />
    )
}
