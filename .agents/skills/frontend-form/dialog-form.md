# A form inside a dialog

Read this only when the form is the content of a `Dialog`, not a page section.

Render `Form` inside `DialogContent` and put the actions in `DialogFooter` so the submit button
stays inside the `<form>`. Control the dialog's open state from the page (`isOpen` /
`onOpenChange`), not from the form, so the dialog can stay open when the submission fails.

See
[src/pages/User/components/CreateApiKeyDialog/CreateApiKeyDialog.tsx](../../../frontend/src/pages/User/components/CreateApiKeyDialog/CreateApiKeyDialog.tsx)
for the full reference: a `DialogRoot` controlled by the parent page, a `Form` with a
`CheckboxGroupField` and a `TextField`, and a `DialogFooter` with a `DialogClose` cancel button next
to the submit button.
