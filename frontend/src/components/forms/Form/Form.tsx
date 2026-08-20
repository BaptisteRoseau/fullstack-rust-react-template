import { zodResolver } from '@hookform/resolvers/zod'
import {
    FormProvider,
    useForm,
    type FieldValues,
    type DefaultValues,
    type UseFormReturn,
} from 'react-hook-form'
import type { z, ZodType } from 'zod'

type FormSchema = ZodType<FieldValues, FieldValues>

type FormMethods<TSchema extends FormSchema> = UseFormReturn<
    z.input<TSchema>,
    unknown,
    z.output<TSchema>
>

export type FormProps<TSchema extends FormSchema> = {
    schema: TSchema
    onSubmit: (values: z.output<TSchema>) => void | Promise<void>
    defaultValues?: DefaultValues<z.input<TSchema>>
    className?: string
    children: (methods: FormMethods<TSchema>) => React.ReactNode
}

export function Form<TSchema extends FormSchema>({
    schema,
    onSubmit,
    defaultValues,
    className,
    children,
}: FormProps<TSchema>) {
    const methods = useForm<z.input<TSchema>, unknown, z.output<TSchema>>({
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
