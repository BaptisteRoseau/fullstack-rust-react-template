# `components/ui/`

The **design system** — reusable, business-agnostic UI primitives following the **ShadCN/UI** pattern:
copied into the repo (not installed), built on **Radix UI** headless primitives, styled with
**Tailwind** via **`cva`** variants merged with **`cn()`** (`@/utils/cn`).

## Per-component folder

```
ui/<name>/
├── <name>.tsx           # PascalCase export, forwardRef + displayName for DOM elements
├── <name>.stories.tsx   # Storybook
├── index.ts             # barrel: `export * from './<name>'`  ← required here (features have none)
└── __tests__/           # Vitest + Testing Library (when it has logic)
```

Generate with `bun run generate` (Plop) → choose `components`.

## Rules

- Tailwind utilities only; variants via `cva`; **always merge the incoming `className` last via `cn()`**.
- Use semantic theme tokens (`bg-primary`, `text-muted-foreground`, `border-input`, `text-destructive`), not raw colors.
- Wrap 3rd-party interactive UI in a Radix primitive (`dialog`, `dropdown`, `drawer`, `switch`). Icons from `lucide-react`.
- `asChild` via Radix `Slot` when the element type is caller-controlled.
- **No business logic** — no `@/features/*` or `api-client` imports.

Notable members: `form/` (React Hook Form + Zod field primitives — see the form skill), `notifications/`
(holds the Zustand toast store — see the state skill), `dialog/confirmation-dialog/` (composition example).

See `.claude/skills/frontend-react-component`.
