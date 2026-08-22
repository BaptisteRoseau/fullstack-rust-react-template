---
name: frontend-design-system
description: How to build or update a domain-agnostic UI primitive in src/design-system (SCSS Module + clsx variants + Radix wrapper + mandatory Storybook story). Use this when creating or editing a button, input, dialog, badge, table or any reusable presentational component.
---

# Design system primitives

`src/design-system/` holds **domain-agnostic** UI primitives. Nothing here knows about users, API
services or contexts — a primitive receives everything through props. Radix supplies accessible
behaviour for the hard widgets; we own all the CSS.

**Every component in this folder has a Storybook story.** That rule keeps the layer honest: if a
component cannot be told in a story without mocking an API, it belongs in `src/components/`.

## Folder shape

Generate it — the story, the stylesheet name and the barrel come out right for free. From
`frontend/`:

```bash
bun run generate component design-system "" <ComponentName>
# e.g. bun run generate component design-system "" Badge
# inside a grouping folder: bun run generate component design-system inputs TextInput
```

The second argument is the grouping folder; pass `""` for none. Run `bun run generate` with no
arguments to be prompted instead. Choosing the `design-system` layer is what adds the mandatory
story to the generated folder.

```
Badge/
├── Badge.tsx               # implementation
├── Badge.test.tsx          # unit test
├── Badge.stories.tsx       # mandatory
├── badge.module.scss       # SCSS Module, kebab-case
└── index.ts                # barrel
```

```ts
// index.ts
export { Badge } from './Badge'
export type { BadgeProps } from './Badge'
```

Lowercase grouping folders (`inputs/`) are allowed when a family grows past three members. They get
no barrel of their own — import the leaf: `@/design-system/inputs/TextInput`.

## Writing a primitive

Variants are modifier classes composed with `clsx`. Always forward `className` and spread the rest
of the native props so callers can extend without a wrapper element.

```tsx
import { Slot } from '@radix-ui/react-slot'
import clsx from 'clsx'

import styles from './button.module.scss'

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
    size?: 'sm' | 'md' | 'lg'
    asChild?: boolean
}

export function Button({
    variant = 'primary',
    size = 'md',
    asChild = false,
    className,
    type = 'button',
    ...props
}: ButtonProps) {
    const Component = asChild ? Slot : 'button'

    return (
        <Component
            className={clsx(styles.button, styles[variant], styles[size], className)}
            type={asChild ? undefined : type}
            {...props}
        />
    )
}
```

`asChild` (Radix `Slot`) is how a router `<Link>` gets button styling without nesting an anchor
inside a button:

```tsx
<Button asChild><Link to={PATHS.login}><Trans>Log in</Trans></Link></Button>
```

For a forwarded input, take `ref` as a normal prop — React 19 needs no `forwardRef`.

## The stylesheet

`src/css` is on the Sass load path, so tokens import without relative chains.

```scss
@use 'variables' as *;
@use 'mixins' as *;

.button {
    display: inline-flex;
    align-items: center;
    gap: $space-2;
    border-radius: $radius-md;
    cursor: pointer;

    &:disabled { opacity: 0.5; cursor: not-allowed; }

    @include focus-ring;
}

.primary {
    background-color: var(--color-primary);
    color: var(--color-on-primary);
}
```

The split is the whole theming strategy:

- **SCSS variables** (`$space-4`, `$radius-md`, `$font-size-lg`, `$breakpoint-md`) for values that
  never change at runtime. They compile away.
- **CSS custom properties** (`var(--color-primary)`) for anything that changes at runtime — every
  colour, because of the light/dark themes in `src/css/_themes.scss`.

Available mixins: `focus-ring`, `visually-hidden`, `media-up($breakpoint)`, `container`,
`required-marker`. Breakpoints run `$breakpoint-xs` (475px) through `$breakpoint-2xl` (1536px).

**Mobile first.** Base rules target the smallest viewport; widen with `@include media-up(...)`.
Never write a `max-width` query to undo a desktop default.

**`container` frames sections, not primitives.** Every top-level section of a page includes it;
a design-system component never does — it fills whatever its parent gives it.

## Wrapping a Radix primitive

Keep Radix's compound shape; do not collapse it into one prop-driven component.

```tsx
import * as RadixDialog from '@radix-ui/react-dialog'

export const DialogRoot = RadixDialog.Root
export const DialogTrigger = RadixDialog.Trigger
export const DialogClose = RadixDialog.Close

export function DialogContent({ title, children, className, ...props }: DialogContentProps) {
    return (
        <RadixDialog.Portal>
            <RadixDialog.Overlay className={styles.overlay} />
            <RadixDialog.Content className={clsx(styles.content, className)} {...props}>
                <RadixDialog.Title className={styles.title}>{title}</RadixDialog.Title>
                {children}
            </RadixDialog.Content>
        </RadixDialog.Portal>
    )
}
```

A `RadixDialog.Title` is mandatory for screen readers. If a dialog is visually title-less, wrap the
title in a `visually-hidden` class rather than omitting it. Style state through Radix data
attributes: `&[data-state='open'] { … }`.

## Icons

SVGs live in `Icon/resources/` and become components through `makeIcon`:

```tsx
import TrashSvg from './resources/trash.svg?react'
export const TrashIcon = makeIcon(TrashSvg, 'TrashIcon')
```

Icons are `aria-hidden` by default — an icon next to a label is decorative. An icon-only control
carries its own `aria-label` on the **button** (`IconButton` requires it), never on the SVG.

## Story

```tsx
import type { Meta, StoryObj } from '@storybook/react-vite'

import { Badge } from './Badge'

const meta = {
    title: 'Design System/Badge',
    component: Badge,
} satisfies Meta<typeof Badge>

export default meta

type Story = StoryObj<typeof meta>

export const Default: Story = { args: { children: 'read' } }
export const AllVariants: Story = {
    args: { children: 'read' },
    render: () => (
        <div style={{ display: 'flex', gap: 8 }}>
            <Badge>neutral</Badge>
            <Badge variant="success">active</Badge>
        </div>
    ),
}
```

Titles are namespaced `Design System/<Component>`; shared components use `Components/<Component>`.
The preview decorator supplies i18n, a router and a light/dark toolbar toggle — check both themes.

## Test

Assert on roles, labels and text — never on class names. SCSS Module hashes are not a contract.
Every assertion carries a message showing the offending value.

```tsx
it('calls onClick when pressed', async () => {
    const onClick = vi.fn()
    render(<Button onClick={onClick}>Save</Button>)

    await userEvent.click(screen.getByRole('button', { name: 'Save' }))

    expect(
        onClick,
        `expected 1 call, got ${onClick.mock.calls.length}`,
    ).toHaveBeenCalledTimes(1)
})
```

Primitives use Testing Library's plain `render` — they need no providers.
