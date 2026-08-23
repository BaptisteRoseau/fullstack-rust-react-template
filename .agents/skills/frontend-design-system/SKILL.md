---
name: frontend-design-system
description: Use when creating or editing a button, input, dialog, badge, table or any reusable presentational component with no domain knowledge.
---

# Design system primitives

`src/design-system/` holds **domain-agnostic** UI primitives. Nothing here knows about users, API
services or contexts — a primitive receives everything through props. Radix supplies accessible
behaviour for the hard widgets; we own all the CSS. See Skill(frontend-architecture) for how this
layer fits the rest, and Skill(frontend-component) for the domain-aware layer above it.

**Every component in this folder has a Storybook story.** That rule keeps the layer honest: if a
component cannot be told in a story without mocking an API, it belongs in `src/components/`.

## 1. Generate the folder

```bash
bun run generate component design-system "" <ComponentName>
# e.g. bun run generate component design-system "" Badge
# inside a grouping folder: bun run generate component design-system inputs TextInput
```

The second argument is the grouping folder; pass `""` for none. Run `bun run generate` with no
arguments to be prompted instead. Choosing the `design-system` layer is what adds the mandatory
story — see [Badge](../../../frontend/src/design-system/Badge) for the resulting shape:
`Badge.tsx`, `Badge.test.tsx`, `Badge.stories.tsx`, `badge.module.scss`, `index.ts`.

Lowercase grouping folders (e.g. `inputs/`) are allowed when a family grows past three members. They
get no barrel of their own — import the leaf: `@/design-system/inputs/TextInput`.

## 2. Write the primitive

Variants are modifier classes composed with `clsx`. Always forward `className` and spread the rest
of the native props so callers can extend without a wrapper element. See
[src/design-system/Button/Button.tsx](../../../frontend/src/design-system/Button/Button.tsx) — its
`asChild` prop (Radix `Slot`) is how a router `<Link>` gets button styling without nesting an anchor
inside a button. For a forwarded input, take `ref` as a normal prop — React 19 needs no
`forwardRef`.

## 3. Write the stylesheet

`src/css` is on the Sass load path, so tokens import without relative chains:

```scss
@use 'variables' as *;
@use 'mixins' as *;
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

**`container` frames sections, not primitives.** Every top-level section of a page includes it; a
design-system component never does — it fills whatever its parent gives it.

If the primitive wraps a compound Radix widget (dialog, dropdown, table), read
[radix-wrapper.md](./radix-wrapper.md) first. If it is an icon, read [icons.md](./icons.md) instead.

## 4. Write the story

Titles are namespaced `Design System/<Component>`; shared components use `Components/<Component>`.
See
[src/design-system/Badge/Badge.stories.tsx](../../../frontend/src/design-system/Badge/Badge.stories.tsx)
for the shape: a default export with `title` and `component`, then one `Story` per state worth
showing. The preview decorator supplies i18n, a router and a light/dark toolbar toggle — check both
themes.

## 5. Write the test

Assert on roles, labels and text — never on class names. SCSS Module hashes are not a contract, and
Vitest does not process CSS, so `styles.button` is `undefined` in tests. Every assertion carries a
message showing the offending value — see
[src/design-system/Button/Button.test.tsx](../../../frontend/src/design-system/Button/Button.test.tsx).
Primitives use Testing Library's plain `render` — they need no providers. Skill(frontend-testing)
covers the assertion style in full.

## Checklist

```bash
bun run storybook      # check the story renders in both light and dark themes
```

- [ ] The component has a story, unless it genuinely cannot render without an API mock.
- [ ] No `api/`, `contexts/`, `components/` or `pages/` import anywhere in the folder.
