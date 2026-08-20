import { useFormContext } from 'react-hook-form'

import { FormField } from '@/components/forms/FormField'
import { TextArea } from '@/design-system/inputs/TextArea'

export type TextAreaFieldProps = {
    name: string
    label: string
    description?: string
    placeholder?: string
    rows?: number
    disabled?: boolean
}

export function TextAreaField({
    name,
    label,
    description,
    ...textAreaProps
}: TextAreaFieldProps) {
    const { register } = useFormContext()

    return (
        <FormField name={name} label={label} description={description}>
            {(fieldProps) => (
                <TextArea
                    {...fieldProps}
                    {...textAreaProps}
                    {...register(name)}
                />
            )}
        </FormField>
    )
}
