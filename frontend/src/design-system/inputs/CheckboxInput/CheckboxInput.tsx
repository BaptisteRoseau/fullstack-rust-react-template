import clsx from 'clsx'

import styles from './checkbox-input.module.scss'

export type CheckboxInputProps = Omit<
    React.InputHTMLAttributes<HTMLInputElement>,
    'type'
> & {
    label: string
}

export const CheckboxInput = ({
    label,
    className,
    ref,
    ...props
}: CheckboxInputProps & { ref?: React.Ref<HTMLInputElement> }) => (
    <label className={clsx(styles.wrapper, className)}>
        <input ref={ref} type="checkbox" className={styles.input} {...props} />
        <span>{label}</span>
    </label>
)
