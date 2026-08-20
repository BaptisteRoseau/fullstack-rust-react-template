import clsx from 'clsx'

import styles from './text-input.module.scss'

export type TextInputProps = React.InputHTMLAttributes<HTMLInputElement>

export const TextInput = ({
    className,
    type = 'text',
    ref,
    ...props
}: TextInputProps & { ref?: React.Ref<HTMLInputElement> }) => (
    <input
        ref={ref}
        type={type}
        className={clsx(styles.input, className)}
        {...props}
    />
)
