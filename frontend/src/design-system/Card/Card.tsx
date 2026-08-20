import clsx from 'clsx'

import styles from './card.module.scss'

export type CardProps = React.HTMLAttributes<HTMLDivElement>

export function Card({ className, ...props }: CardProps) {
    return <div className={clsx(styles.card, className)} {...props} />
}
