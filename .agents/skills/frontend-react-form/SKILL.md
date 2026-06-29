---
name: frontend-react-form
description: How to build or update a form using the shared Form primitives (React Hook Form + Zod schema + ui/form fields, often inside a FormDrawer). Use this when creating or updating a frontend form, input flow, or create/edit dialog.
---

# Forms

Forms compose the primitives in `src/components/ui/form` — **React Hook Form** driven by a **Zod**
schema. The `<Form>` component owns the `useForm` + `zodResolver` wiring; fields are bound via a
`registration` prop. The Zod schema is **reused from the API file** (`createThingInputSchema`), so
validation and the request type stay in sync.

## Standard create/edit flow (FormDrawer)

```tsx
import { t, Trans } from '@lingui/macro'
import { Plus } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Form, FormDrawer, Input, Textarea } from '@/components/ui/form'
import { useNotifications } from '@/components/ui/notifications'

import { createThingInputSchema, useCreateThing } from '../api/create-thing'

export const CreateThing = () => {
    const { addNotification } = useNotifications()
    const createThing = useCreateThing({
        mutationConfig: {
            onSuccess: () => addNotification({ type: 'success', title: t`Thing Created` }),
        },
    })

    return (
        <FormDrawer
            isDone={createThing.isSuccess}
            triggerButton={
                <Button size="sm" icon={<Plus className="size-4" />}>
                    <Trans>Create Thing</Trans>
                </Button>
            }
            title={t`Create Thing`}
            submitButton={
                <Button form="create-thing" type="submit" size="sm" isLoading={createThing.isPending}>
                    <Trans>Submit</Trans>
                </Button>
            }
        >
            <Form
                id="create-thing"
                schema={createThingInputSchema}
                onSubmit={(values) => createThing.mutate({ data: values })}
            >
                {({ register, formState }) => (
                    <>
                        <Input label={t`Title`} error={formState.errors['title']} registration={register('title')} />
                        <Textarea label={t`Body`} error={formState.errors['body']} registration={register('body')} />
                    </>
                )}
            </Form>
        </FormDrawer>
    )
}
```

## Rules specific to this repo

- **`<Form>` takes `schema` + `onSubmit`** and exposes RHF methods via a **render-prop child** `({ register, formState }) => ...`. Don't call `useForm` yourself in the component.
- **Reuse the Zod schema from the api file** — never redefine field validation in the component. The schema's inferred type is the mutation payload.
- **Bind fields with `registration={register('field')}`** and pass `error={formState.errors['field']}`. Use the wrapped fields (`Input`, `Textarea`, `Select`, `Switch`) from `@/components/ui/form`, which render labels + errors via `FieldWrapper` — don't use bare `<input>`.
- **FormDrawer / submit button:** the submit `<Button>` lives in `submitButton` and links to the form by `form="<id>"` + `type="submit"`. Set `isDone={mutation.isSuccess}` so the drawer auto-closes; `isLoading={mutation.isPending}`.
- **Toast on success** via `useNotifications().addNotification`; the api-client interceptor already toasts errors.
- Wrap mutating forms in `<Authorization>` when the action is role-gated (`frontend-react-authorization`).
- Labels/buttons use Lingui macros (`frontend-react-i18n`).
