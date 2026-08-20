import clsx from 'clsx'

import { Button, type ButtonProps } from './Button'
import styles from './button.module.scss'

export type IconButtonProps = Omit<ButtonProps, 'asChild'> & {
    'aria-label': string
}

export function IconButton({ className, ...props }: IconButtonProps) {
    return <Button className={clsx(styles.icon, className)} {...props} />
}
