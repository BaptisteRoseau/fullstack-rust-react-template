---
name: frontend-form
description: Use when creating or updating a frontend form, input flow, create/edit dialog or validation rule.
---

# Forms

Forms are React Hook Form + Zod. `src/components/forms/` binds them to the design-system inputs so
pages never touch RHF's low-level API. See
[src/components/forms](../../../frontend/src/components/forms) for the current shape: `Form/`,
`FormField/` and `fields/` (`TextField`, `TextAreaField`, `CheckboxGroupField`).

## 1. Write the schema

Put it next to the form that uses it — in the page folder for a page-specific form, in the
component folder for a reusable one. A schema describing an API payload lives in
`src/api/domains/<domain>/types.ts` instead — Skill(frontend-api).

```ts
const createApiKeySchema = z.object({
    name: z.string().min(1),
    permissions: z.array(z.enum(API_KEY_PERMISSIONS)).min(1),
})
```

## 2. Render the form

`Form` owns the `<form>` element, `FormProvider` and the `zodResolver`. It takes a render-prop
child so a caller that needs RHF state (`watch`, `formState`) can reach it without prop drilling —
most forms ignore the argument. See
[src/pages/User/components/CreateApiKeyDialog/CreateApiKeyDialog.tsx](../../../frontend/src/pages/User/components/CreateApiKeyDialog/CreateApiKeyDialog.tsx)
for the full pattern: schema, fields, submit handler and dialog footer together.

## 3. Wire the submit handler to a mutation hook

The hook owns its cache invalidation, so the handler only reports the outcome:

```tsx
async function handleSubmit(values: z.infer<typeof createApiKeySchema>) {
    try {
        onCreated(await trigger(values))
    } catch (error) {
        addNotification({ type: 'error', title: t`Could not create the API key`, message: apiErrorMessage(error) })
    }
}
```

`trigger` rejects on failure. Never swallow it. See Skill(frontend-state) for `addNotification` and
Skill(frontend-api) for `apiErrorMessage`.

## 4. Add a field component, if the control doesn't exist yet

Generate it as a shared component under the `forms/fields` grouping:

```bash
bun run generate component components forms/fields <Name>Field
```

Every field builds on `FormField`, which owns the label, the description, the error message and the
`aria-describedby`/`aria-invalid` wiring. See
[src/components/forms/fields/TextField/TextField.tsx](../../../frontend/src/components/forms/fields/TextField/TextField.tsx):
spread order matters, `register(name)` goes **last**, so RHF's `ref`, `name`, `onChange` and
`onBlur` are never overwritten.

The control itself is a design-system input — a field component never writes raw markup or CSS for
the control. If the control shape does not exist yet, add the primitive to
`src/design-system/inputs/` first — Skill(frontend-design-system) — then bind it here.

If the form lives inside a dialog, read [dialog-form.md](./dialog-form.md).

## Rules

- `noValidate` is set by `Form`; validation is Zod's, so messages are consistent and translatable.
- Never disable a submit button on `!isValid` — disable it on `isMutating` only. A user must be
  able to submit and see what is wrong.
- Label every control. A `TextField` without a `label` is a bug, not a style choice.
- Errors render with `role="alert"`, so tests and e2e assert on the role, not on a class.

## Checklist

- [ ] The submit button disables on `isMutating`, never on `!isValid`.
- [ ] Every field has a `label`.
