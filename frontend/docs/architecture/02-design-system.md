# 02 – Design system

← [Back to overview](README.md)

`src/design-system/` holds presentational, **domain-agnostic** UI primitives. Nothing here knows
about users, API services or application contexts — a primitive receives everything through props.
Radix UI supplies accessible behaviour for the hard widgets; we own all the CSS.

**Every component in this folder has a Storybook story.** That is the rule that keeps the layer
honest: if a component can't be told in a story without mocking an API, it belongs in
[`components/`](03-components.md) instead.

---

## Directory tree

```
src/design-system/
├── Avatar/
├── Badge/
│   ├── Badge.tsx
│   ├── Badge.test.tsx
│   ├── Badge.stories.tsx       # mandatory
│   ├── badge.module.scss
│   ├── index.ts
│   └── __snapshots__/
│
├── Button/
│   ├── Button.tsx              # base button, variant + size props
│   ├── IconButton.tsx          # icon-only variant
│   ├── Button.stories.tsx
│   ├── button.module.scss
│   └── index.ts
│
├── Card/
├── Dialog/                     # Radix Dialog
│   ├── Dialog.tsx
│   ├── DialogHeader.tsx
│   ├── DialogFooter.tsx
│   ├── dialog.module.scss
│   └── index.ts
├── Drawer/                     # Radix Dialog, side-anchored
├── Dropdown/                   # Radix DropdownMenu
├── Icon/
│   ├── Icon.tsx
│   ├── makeIcon.tsx            # SVG → component factory
│   └── resources/              # .svg files
├── inputs/                     # Low-level form controls
│   ├── CheckboxInput/
│   ├── SelectInput/
│   ├── SwitchInput/            # Radix Switch
│   ├── TextArea/
│   └── TextInput/
├── Link/
├── Pagination/
├── ProgressBar/
├── Spinner/
├── Table/
│   ├── Table.tsx
│   ├── TableHeader.tsx
│   ├── TableRow.tsx
│   └── index.ts
├── Tabs/                       # Radix Tabs
├── Tag/
└── Tooltip/
```

Lowercase grouping folders (`inputs/`) are allowed when a family grows past three members. They get
no `index.ts` of their own — import the leaf: `@/design-system/inputs/TextInput`.

---

## Component folder structure

```
Badge/
├── Badge.tsx               # implementation
├── Badge.test.tsx          # unit test
├── Badge.stories.tsx       # mandatory Storybook story
├── badge.module.scss       # SCSS Module, kebab-case
├── index.ts                # barrel
└── __snapshots__/          # only if the test uses toMatchSnapshot
```

```ts
// index.ts
export { Badge } from './Badge';
export type { BadgeProps } from './Badge';
```

---

## Writing a primitive

Variants are modifier classes composed with `clsx`. Always forward `className` and spread the rest
of the native props so callers can extend without a wrapper element.

```tsx
// src/design-system/Button/Button.tsx
import clsx from 'clsx';
import { Slot } from '@radix-ui/react-slot';

import styles from './button.module.scss';

export type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> & {
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    /** Render the child element instead of a <button>, keeping the styles. */
    asChild?: boolean;
};

export function Button({
    variant = 'primary',
    size = 'md',
    asChild = false,
    className,
    type = 'button',
    ...props
}: ButtonProps) {
    const Component = asChild ? Slot : 'button';

    return (
        <Component
            className={clsx(styles.button, styles[variant], styles[size], className)}
            type={asChild ? undefined : type}
            {...props}
        />
    );
}
```

`asChild` (Radix `Slot`) is how a router `<Link>` gets button styling without nesting an anchor in
a button.

### The stylesheet

Tokens come from `src/css/`, which is on the Sass load path — see
[06 – Tooling](06-tooling.md#global-styles-and-tokens).

```scss
// src/design-system/Button/button.module.scss
@use 'variables' as *;
@use 'mixins' as *;

.button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: $space-2;
    border: 1px solid transparent;
    border-radius: $radius-md;
    font-weight: 500;
    cursor: pointer;
    transition: background-color $duration-fast ease;

    &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    @include focus-ring;
}

/* Variants */
.primary {
    background-color: var(--color-primary);
    color: var(--color-on-primary);

    &:hover:not(:disabled) {
        background-color: var(--color-primary-hover);
    }
}

.secondary {
    background-color: transparent;
    border-color: var(--color-border);
    color: var(--color-text);
}

.ghost { background-color: transparent; color: var(--color-text); }
.danger { background-color: var(--color-danger); color: var(--color-on-danger); }

/* Sizes */
.sm { padding: $space-1 $space-2; font-size: $font-size-sm; }
.md { padding: $space-2 $space-3; font-size: $font-size-base; }
.lg { padding: $space-3 $space-4; font-size: $font-size-lg; }
```

Static values (spacing, radii, font sizes) are **SCSS variables** — they are compile-time and cost
nothing. Anything that changes at runtime (colours, because of theming) is a **CSS custom
property**. That split is the whole theming strategy.

---

## Wrapping a Radix primitive

Keep Radix's compound-component shape; do not collapse it into a single monolithic prop-driven
component. Consumers get composition, we only supply the styling.

```tsx
// src/design-system/Dialog/Dialog.tsx
import * as RadixDialog from '@radix-ui/react-dialog';
import clsx from 'clsx';

import styles from './dialog.module.scss';

export const DialogRoot = RadixDialog.Root;
export const DialogTrigger = RadixDialog.Trigger;
export const DialogClose = RadixDialog.Close;

type DialogContentProps = React.ComponentPropsWithoutRef<typeof RadixDialog.Content> & {
    title: string;
};

export function DialogContent({ title, children, className, ...props }: DialogContentProps) {
    return (
        <RadixDialog.Portal>
            <RadixDialog.Overlay className={styles.overlay} />
            <RadixDialog.Content className={clsx(styles.content, className)} {...props}>
                <RadixDialog.Title className={styles.title}>{title}</RadixDialog.Title>
                {children}
            </RadixDialog.Content>
        </RadixDialog.Portal>
    );
}
```

Radix ships no CSS beyond inline positioning for floating elements, so the module stylesheet owns
appearance entirely. Use its data attributes for state-driven styles:

```scss
.content {
    &[data-state='open'] { animation: fade-in $duration-fast ease-out; }
    &[data-state='closed'] { animation: fade-out $duration-fast ease-in; }
}
```

A `RadixDialog.Title` is mandatory for screen readers — if a dialog is visually title-less, wrap
the title in a `.visuallyHidden` class rather than omitting it.

---

## Icon factory

```tsx
// src/design-system/Icon/makeIcon.tsx
export function makeIcon(Svg: React.FC<React.SVGProps<SVGSVGElement>>, displayName: string) {
    function Icon({ size = 16, ...props }: React.SVGProps<SVGSVGElement> & { size?: number }) {
        return <Svg width={size} height={size} aria-hidden focusable={false} {...props} />;
    }
    Icon.displayName = displayName;
    return Icon;
}

// Usage — requires vite-plugin-svgr for the ?react suffix
import AddSvg from './resources/add.svg?react';
export const AddIcon = makeIcon(AddSvg, 'AddIcon');
```

Icons are `aria-hidden` by default: an icon next to a label is decorative. An icon-only control
carries its own `aria-label` on the *button*, not on the SVG.

---

## Story

```tsx
// src/design-system/Badge/Badge.stories.tsx
import type { Meta, StoryObj } from '@storybook/react-vite';

import { Badge } from './Badge';

const meta = {
    component: Badge,
    title: 'Design System/Badge',
} satisfies Meta<typeof Badge>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { args: { children: 'New' } };
export const Success: Story = { args: { children: 'Published', variant: 'success' } };
export const AllVariants: Story = {
    render: () => (
        <>
            <Badge>Default</Badge>
            <Badge variant="success">Published</Badge>
            <Badge variant="warning">Draft</Badge>
        </>
    ),
};
```

Titles are namespaced `Design System/<Component>`; shared components use `Components/<Component>`.

---

## Test

Assert on roles and text, not on class names — SCSS Module hashes are not a stable contract, and
Vitest does not process CSS by default (see
[06 – Tooling](06-tooling.md#css-modules-in-vitest)).

```tsx
// src/design-system/Button/Button.test.tsx
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { Button } from './Button';

it('calls onClick when pressed', async () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Save</Button>);

    await userEvent.click(screen.getByRole('button', { name: 'Save' }));

    expect(onClick, `expected 1 call, got ${onClick.mock.calls.length}`).toHaveBeenCalledTimes(1);
});

it('does not fire when disabled', async () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick} disabled>Save</Button>);

    await userEvent.click(screen.getByRole('button', { name: 'Save' }));

    expect(onClick, `expected no calls, got ${onClick.mock.calls.length}`).not.toHaveBeenCalled();
});
```
