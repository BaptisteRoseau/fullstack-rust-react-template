import * as RadixDialog from '@radix-ui/react-dialog'
import clsx from 'clsx'

import styles from './dialog.module.scss'

export const DialogRoot = RadixDialog.Root
export const DialogTrigger = RadixDialog.Trigger
export const DialogClose = RadixDialog.Close

export type DialogContentProps = React.ComponentPropsWithoutRef<
    typeof RadixDialog.Content
> & {
    title: string
    description?: string
}

export function DialogContent({
    title,
    description,
    children,
    className,
    ...props
}: DialogContentProps) {
    return (
        <RadixDialog.Portal>
            <RadixDialog.Overlay className={styles.overlay} />
            <RadixDialog.Content
                className={clsx(styles.content, className)}
                {...props}
            >
                <RadixDialog.Title className={styles.title}>
                    {title}
                </RadixDialog.Title>
                {description ? (
                    <RadixDialog.Description className={styles.description}>
                        {description}
                    </RadixDialog.Description>
                ) : null}
                {children}
            </RadixDialog.Content>
        </RadixDialog.Portal>
    )
}

export function DialogFooter({
    className,
    ...props
}: React.HTMLAttributes<HTMLDivElement>) {
    return <div className={clsx(styles.footer, className)} {...props} />
}
