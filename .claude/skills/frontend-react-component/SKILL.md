---
name: frontend-react-component
description: How to create or update a shared UI component under src/components/ui (Tailwind + cva + cn, Radix primitives, ShadCN pattern). Use this when building or editing a reusable frontend component, button, dialog, input, or design-system primitive.
---

# Shared UI Component

Reusable, app-agnostic primitives live in `src/components/ui/<name>/`. They follow the **ShadCN/UI
pattern**: components are copied into the repo (not installed), built on **Radix UI** headless
primitives, styled with **Tailwind** via **`cva`** variants and merged with **`cn()`**.

Generate the scaffold with `bun run generate` (Plop) — pick `components`, give a folder name. It
creates `<name>.tsx`, `<name>.stories.tsx`, and `index.ts`.

## Directory layout

```
src/components/ui/<name>/
├── <name>.tsx           # component (PascalCase export, kebab-case file)
├── <name>.stories.tsx   # Storybook stories (see the frontend-storybook skill)
├── index.ts             # barrel: `export * from './<name>'`  ← required for ui/*
└── __tests__/           # Vitest + Testing Library (when it has logic)
```

`components/ui/*` is the ONE place barrels are required — features have none.

## Skeleton (variants + cn + Radix Slot polymorphism)

```tsx
import { Slot } from '@radix-ui/react-slot'
import { cva, type VariantProps } from 'class-variance-authority'
import * as React from 'react'

import { cn } from '@/utils/cn'

const thingVariants = cva('inline-flex items-center rounded-md text-sm', {
    variants: {
        variant: { default: 'bg-primary text-primary-foreground', outline: 'border border-input' },
        size: { default: 'h-9 px-4 py-2', sm: 'h-8 px-3 text-xs' },
    },
    defaultVariants: { variant: 'default', size: 'default' },
})

export type ThingProps = React.HTMLAttributes<HTMLDivElement> &
    VariantProps<typeof thingVariants> & { asChild?: boolean }

const Thing = React.forwardRef<HTMLDivElement, ThingProps>(
    ({ className, variant, size, asChild = false, ...props }, ref) => {
        const Comp = asChild ? Slot : 'div'
        return <Comp ref={ref} className={cn(thingVariants({ variant, size, className }))} {...props} />
    },
)
Thing.displayName = 'Thing'

export { Thing, thingVariants }
```

## Rules

- **Styling:** Tailwind utility classes only. Variants via `cva`. **Always** merge the incoming `className` last through `cn()` (from `@/utils/cn` — clsx + tailwind-merge) so callers can override.
- **Wrap, don't inline, 3rd-party UI.** New interactive primitives wrap a Radix component (see `dialog`, `dropdown`, `drawer`, `switch`). Icons come from `lucide-react`.
- **`forwardRef` + `displayName`** for anything that renders a DOM element callers may ref.
- **`asChild` via Radix `Slot`** when the element type should be caller-controlled (links/buttons).
- **Theme tokens** (`bg-primary`, `text-muted-foreground`, `border-input`, `text-destructive`) — use the semantic token classes, not raw colors, so theming works.
- **Composition over props.** Many props → split (see `dialog/confirmation-dialog`). Use `children`/slots.
- Add a Storybook story (`frontend-storybook`) and, if it has logic, a test (`frontend-react-testing`).
- Keep these **business-agnostic** — no `@/features/*` or `@/lib/api-client` imports. Business UI belongs in a feature.
