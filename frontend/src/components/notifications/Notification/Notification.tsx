import { useLingui } from '@lingui/react/macro'
import clsx from 'clsx'

import { IconButton } from '@/design-system/Button'
import { CloseIcon } from '@/design-system/Icon'
import type { Notification as NotificationData } from '@/stores/notifications'

import styles from './notification.module.scss'

export type NotificationProps = {
    notification: NotificationData
    onDismiss: (id: string) => void
}

export function Notification({ notification, onDismiss }: NotificationProps) {
    const { t } = useLingui()

    return (
        <div
            role="alert"
            aria-label={notification.title}
            className={clsx(styles.notification, styles[notification.type])}
        >
            <div className={styles.body}>
                <p className={styles.title}>{notification.title}</p>
                {notification.message ? <p>{notification.message}</p> : null}
            </div>
            <IconButton
                aria-label={t`Close`}
                variant="ghost"
                size="sm"
                onClick={() => onDismiss(notification.id)}
            >
                <CloseIcon />
            </IconButton>
        </div>
    )
}
