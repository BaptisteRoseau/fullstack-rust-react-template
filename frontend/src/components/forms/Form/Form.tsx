import { zodResolver } from '@hookform/resolvers/zod'
import {
    FormProvider,
    useForm,
    type FieldValues,
    type DefaultValues,
    type UseFormReturn,
} from 'react-hook-form'
import type { ZodType } from 'zod'

type FormMethods<
    TInput extends FieldValues,
    TOutput extends FieldValues,
> = UseFormReturn<TInput, unknown, TOutput>

export type FormProps<
    TInput extends FieldValues,
    TOutput extends FieldValues,
> = {
    schema: ZodType<TOutput, TInput>
    onSubmit: (values: TOutput) => void | Promise<void>
    defaultValues?: DefaultValues<TInput>
    className?: string
    children: (methods: FormMethods<TInput, TOutput>) => React.ReactNode
}

export function Form<TInput extends FieldValues, TOutput extends FieldValues>({
    schema,
    onSubmit,
    defaultValues,
    className,
    children,
}: FormProps<TInput, TOutput>) {
    const methods = useForm<TInput, unknown, TOutput>({
        resolver: zodResolver(schema),
        defaultValues,
    })

    return (
        <FormProvider {...methods}>
            <form
                className={className}
                onSubmit={methods.handleSubmit(onSubmit)}
                noValidate
            >
                {children(methods)}
            </form>
        </FormProvider>
    )
}
