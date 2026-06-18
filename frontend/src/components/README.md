# `components/`

**Shared, app-agnostic** UI used across the whole app. These import only from other shared modules —
**never** from `@/features/*` or the API client. Business-specific UI belongs in a feature instead.

| Folder | Contents |
|--------|----------|
| `ui/` | The design system: `button`, `dialog` (+ `confirmation-dialog`), `drawer`, `dropdown`, `form` (input/textarea/select/switch/label/field-wrapper/error/form-drawer), `table` (+ pagination), `link`, `spinner`, `md-preview`, `notifications`. |
| `layouts/` | Page shells: `content-layout`, `auth-layout`, `dashboard-layout`. |
| `errors/` | `MainErrorFallback` used by the root error boundary. |
| `seo/` | `Head` (wraps `react-helmet-async`) for per-page document titles. |

## Conventions

- Built with **Tailwind + `cva` + `cn()`** on top of **Radix UI** primitives (ShadCN pattern: copied in, not installed).
- Each `ui/*` component is a folder with `<name>.tsx`, `<name>.stories.tsx`, an `index.ts` barrel, and `__tests__/` when it has logic.
- Scaffold with `bun run generate` (Plop).

See `.claude/skills/frontend-react-component` and `.claude/skills/frontend-react-layout`.
