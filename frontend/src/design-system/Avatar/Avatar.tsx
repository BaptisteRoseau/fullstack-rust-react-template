import clsx from 'clsx'

import { initials } from '@/utils/strings'

import styles from './avatar.module.scss'

export type AvatarProps = React.HTMLAttributes<HTMLSpanElement> & {
    name: string
}

export function Avatar({ name, className, ...props }: AvatarProps) {
    return (
        <span className={clsx(styles.avatar, className)} {...props}>
            {initials(name)}
        </span>
    )
}
