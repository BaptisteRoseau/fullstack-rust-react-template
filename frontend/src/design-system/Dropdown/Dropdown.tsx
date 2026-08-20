import * as RadixDropdown from '@radix-ui/react-dropdown-menu'
import clsx from 'clsx'

import styles from './dropdown.module.scss'

export const Dropdown = RadixDropdown.Root
export const DropdownTrigger = RadixDropdown.Trigger

export type DropdownContentProps = React.ComponentPropsWithoutRef<
    typeof RadixDropdown.Content
>

export function DropdownContent({
    className,
    align = 'end',
    sideOffset = 6,
    ...props
}: DropdownContentProps) {
    return (
        <RadixDropdown.Portal>
            <RadixDropdown.Content
                align={align}
                sideOffset={sideOffset}
                className={clsx(styles.content, className)}
                {...props}
            />
        </RadixDropdown.Portal>
    )
}

export function DropdownItem({
    className,
    ...props
}: React.ComponentPropsWithoutRef<typeof RadixDropdown.Item>) {
    return (
        <RadixDropdown.Item
            className={clsx(styles.item, className)}
            {...props}
        />
    )
}

export function DropdownLabel({
    className,
    ...props
}: React.ComponentPropsWithoutRef<typeof RadixDropdown.Label>) {
    return (
        <RadixDropdown.Label
            className={clsx(styles.label, className)}
            {...props}
        />
    )
}

export function DropdownSeparator({
    className,
    ...props
}: React.ComponentPropsWithoutRef<typeof RadixDropdown.Separator>) {
    return (
        <RadixDropdown.Separator
            className={clsx(styles.separator, className)}
            {...props}
        />
    )
}
