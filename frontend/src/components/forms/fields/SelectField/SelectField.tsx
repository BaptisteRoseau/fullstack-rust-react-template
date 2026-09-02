import { useFormContext } from 'react-hook-form'

import { FormField } from '@/components/forms/FormField'
import {
    SelectInput,
    type SelectOption,
} from '@/design-system/inputs/SelectInput'

export type SelectFieldProps = {
    name: string
    label: string
    description?: string
    options: SelectOption[]
    disabled?: boolean
}

export function SelectField({
    name,
    label,
    description,
    ...inputProps
}: SelectFieldProps) {
    const { register } = useFormContext()

    return (
        <FormField name={name} label={label} description={description}>
            {(fieldProps) => (
                <SelectInput
                    {...fieldProps}
                    {...inputProps}
                    {...register(name)}
                />
            )}
        </FormField>
    )
}
