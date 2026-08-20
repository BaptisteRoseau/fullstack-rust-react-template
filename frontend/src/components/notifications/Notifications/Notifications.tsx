import { Notification } from '@/components/notifications/Notification'
import { useNotifications } from '@/stores/notifications'

import styles from './notifications.module.scss'

export function Notifications() {
    const notifications = useNotifications((state) => state.notifications)
    const dismissNotification = useNotifications(
        (state) => state.dismissNotification,
    )

    if (notifications.length === 0) {
        return null
    }

    return (
        <div className={styles.stack}>
            {notifications.map((notification) => (
                <Notification
                    key={notification.id}
                    notification={notification}
                    onDismiss={dismissNotification}
                />
            ))}
        </div>
    )
}
