import { useId } from 'react'
import { useFormContext } from 'react-hook-form'

import styles from './form-field.module.scss'

export type FormFieldChildProps = {
    id: string
    'aria-invalid': boolean
    'aria-describedby': string | undefined
}

export type FormFieldProps = {
    name: string
    label: string
    description?: string
    children: (props: FormFieldChildProps) => React.ReactNode
}

export function FormField({
    name,
    label,
    description,
    children,
}: FormFieldProps) {
    const id = useId()
    const errorId = `${id}-error`
    const descriptionId = `${id}-description`
    const {
        formState: { errors },
    } = useFormContext()

    const error = errors[name]
    const message = typeof error?.message === 'string' ? error.message : null

    return (
        <div className={styles.field}>
            <label className={styles.label} htmlFor={id}>
                {label}
            </label>
            {children({
                id,
                'aria-invalid': Boolean(message),
                'aria-describedby': message
                    ? errorId
                    : description
                      ? descriptionId
                      : undefined,
            })}
            {description ? (
                <p id={descriptionId} className={styles.description}>
                    {description}
                </p>
            ) : null}
            {message ? (
                <p id={errorId} role="alert" className={styles.error}>
                    {message}
                </p>
            ) : null}
        </div>
    )
}
