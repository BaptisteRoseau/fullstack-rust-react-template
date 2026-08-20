import * as RadixSwitch from '@radix-ui/react-switch'
import clsx from 'clsx'

import styles from './switch-input.module.scss'

export type SwitchInputProps = React.ComponentPropsWithoutRef<
    typeof RadixSwitch.Root
>

export function SwitchInput({ className, ...props }: SwitchInputProps) {
    return (
        <RadixSwitch.Root className={clsx(styles.root, className)} {...props}>
            <RadixSwitch.Thumb className={styles.thumb} />
        </RadixSwitch.Root>
    )
}
