---
name: frontend-form
description: How to build or update a form with the shared Form primitives (React Hook Form + Zod schema + FormField-bound fields). Use this when creating or updating a frontend form, input flow, create/edit dialog or validation rule.
---

# Forms

Forms are React Hook Form + Zod. `src/components/forms/` binds them to the design-system inputs so
pages never touch RHF's low-level API.

```
src/components/forms/
├── Form/                       # <form> + FormProvider + zodResolver
├── FormField/                  # label + control + error, wired to RHF
└── fields/
    ├── TextField/
    ├── TextAreaField/
    └── CheckboxGroupField/
```

## Writing a form

The schema lives next to the form that uses it — in the page folder for a page-specific form, in
the component folder for a reusable one. Schemas describing an API payload live in
`src/api/<domain>.ts` instead.

```tsx
import * as z from 'zod'

const profileSchema = z.object({
    firstName: z.string().min(1),
    lastName: z.string().min(1),
    bio: z.string().max(500),
})

<Form
    schema={profileSchema}
    onSubmit={handleSubmit}
    className={styles.form}
    defaultValues={{ firstName: user.firstName, lastName: user.lastName, bio: user.bio }}
>
    {() => (
        <>
            <TextField name="firstName" label={t`First name`} />
            <TextField name="lastName" label={t`Last name`} />
            <TextAreaField name="bio" label={t`Bio`} />
            <Button type="submit" disabled={isMutating}>
                <Trans>Save changes</Trans>
            </Button>
        </>
    )}
</Form>
```

`Form` takes a render-prop child so a caller that needs RHF state (`watch`, `formState`) can reach
it without prop drilling. Most forms ignore the argument.

## Submitting

Wire the submit handler to a mutation from `api/service/`, revalidate, and report both outcomes:

```tsx
async function handleSubmit(values: z.infer<typeof profileSchema>) {
    try {
        await trigger(values)
        await mutate(ME_ENDPOINT)
        addNotification({ type: 'success', title: t`Profile updated` })
    } catch (error) {
        addNotification({
            type: 'error',
            title: t`Could not update the profile`,
            message: apiErrorMessage(error, t`Unexpected error`),
        })
    }
}
```

`trigger` rejects on failure. Never swallow it.

## Adding a field component

Generate the folder first — a field is a shared component under the `forms/fields` grouping. From
`frontend/`:

```bash
bun run generate component components forms/fields <Name>Field
# e.g. bun run generate component components forms/fields TextAreaField
```

Every field builds on `FormField`, which owns the label, the description, the error message and the
`aria-describedby`/`aria-invalid` wiring — so accessibility is handled once.

```tsx
export function TextField({ name, label, description, ...inputProps }: TextFieldProps) {
    const { register } = useFormContext()

    return (
        <FormField name={name} label={label} description={description}>
            {(fieldProps) => <TextInput {...fieldProps} {...inputProps} {...register(name)} />}
        </FormField>
    )
}
```

Spread order matters: `register(name)` **last**, so RHF's `ref`, `name`, `onChange` and `onBlur`
are never overwritten.

The control itself is a design-system input — a field component never writes raw markup or CSS for
the control. If a new control shape is needed, add the primitive to `src/design-system/inputs/`
first (see the `frontend-design-system` skill), then bind it here.

## Forms in a dialog

Render `Form` inside `DialogContent` and put the actions in `DialogFooter` so the submit button
stays inside the `<form>`:

```tsx
<DialogRoot open={isOpen} onOpenChange={onOpenChange}>
    <DialogContent title={t`New API key`} description={t`The secret is displayed once.`}>
        <Form schema={createApiKeySchema} onSubmit={handleSubmit}>
            {() => (
                <>
                    <TextField name="name" label={t`Name`} />
                    <DialogFooter>
                        <DialogClose asChild>
                            <Button variant="secondary"><Trans>Cancel</Trans></Button>
                        </DialogClose>
                        <Button type="submit" disabled={isMutating}><Trans>Create key</Trans></Button>
                    </DialogFooter>
                </>
            )}
        </Form>
    </DialogContent>
</DialogRoot>
```

Control the dialog from the page (`isOpen` / `onOpenChange`) so the form can stay open when the
submission fails.

## Rules

- `noValidate` is set by `Form`; validation is Zod's, so messages are consistent and translatable.
- Never disable a submit button on `!isValid` — disable it on `isMutating` only. A user must be
  able to submit and see what is wrong.
- Label every control. A `TextField` without a `label` is a bug, not a style choice.
- Errors render with `role="alert"`, so tests and e2e assert on the role, not on a class.
