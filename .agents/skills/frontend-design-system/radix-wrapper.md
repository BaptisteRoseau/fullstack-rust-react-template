# Wrapping a compound Radix primitive

Read this only when the primitive wraps a multi-part Radix widget (dialog, dropdown, table) rather
than a single element like a badge or a button.

Keep Radix's compound shape; do not collapse it into one prop-driven component. Re-export the parts
that need no styling as-is, and wrap only the ones that need a class or extra markup:

```tsx
export const DialogRoot = RadixDialog.Root       // plain re-export: no styling needed
export const DialogTrigger = RadixDialog.Trigger

export function DialogContent({ title, children, ...props }: DialogContentProps) {
    // wrapped: adds the overlay, the class name and a mandatory title
}
```

See [src/design-system/Dialog/Dialog.tsx](../../../frontend/src/design-system/Dialog/Dialog.tsx)
for the full file.

A `RadixDialog.Title` is mandatory for screen readers. If a dialog is visually title-less, wrap the
title in a `visually-hidden` class rather than omitting it. Style state through Radix data
attributes: `&[data-state='open'] { … }`.
