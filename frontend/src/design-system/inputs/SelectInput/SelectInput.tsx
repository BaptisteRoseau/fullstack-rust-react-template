import clsx from 'clsx'

import styles from './select-input.module.scss'

export type SelectOption = { value: string; label: string }

export type SelectInputProps = React.SelectHTMLAttributes<HTMLSelectElement> & {
    options: SelectOption[]
}

export const SelectInput = ({
    options,
    className,
    ref,
    ...props
}: SelectInputProps & { ref?: React.Ref<HTMLSelectElement> }) => (
    <select ref={ref} className={clsx(styles.select, className)} {...props}>
        {options.map((option) => (
            <option key={option.value} value={option.value}>
                {option.label}
            </option>
        ))}
    </select>
)
