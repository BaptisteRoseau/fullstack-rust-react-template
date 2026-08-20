import { useFormContext } from 'react-hook-form'

import { FormField } from '@/components/forms/FormField'
import { CheckboxInput } from '@/design-system/inputs/CheckboxInput'

import styles from './checkbox-group-field.module.scss'

export type CheckboxGroupFieldProps = {
    name: string
    label: string
    description?: string
    options: { value: string; label: string }[]
}

export function CheckboxGroupField({
    name,
    label,
    description,
    options,
}: CheckboxGroupFieldProps) {
    const { register } = useFormContext()

    return (
        <FormField name={name} label={label} description={description}>
            {() => (
                <div className={styles.group}>
                    {options.map((option) => (
                        <CheckboxInput
                            key={option.value}
                            label={option.label}
                            value={option.value}
                            {...register(name)}
                        />
                    ))}
                </div>
            )}
        </FormField>
    )
}
