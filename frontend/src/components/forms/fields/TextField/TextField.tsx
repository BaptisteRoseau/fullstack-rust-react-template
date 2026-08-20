import { useFormContext } from 'react-hook-form'

import { FormField } from '@/components/forms/FormField'
import { TextInput } from '@/design-system/inputs/TextInput'

export type TextFieldProps = {
    name: string
    label: string
    description?: string
    type?: React.HTMLInputTypeAttribute
    placeholder?: string
    disabled?: boolean
}

export function TextField({
    name,
    label,
    description,
    ...inputProps
}: TextFieldProps) {
    const { register } = useFormContext()

    return (
        <FormField name={name} label={label} description={description}>
            {(fieldProps) => (
                <TextInput
                    {...fieldProps}
                    {...inputProps}
                    {...register(name)}
                />
            )}
        </FormField>
    )
}
