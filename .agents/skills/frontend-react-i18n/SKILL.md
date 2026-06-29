---
name: frontend-react-i18n
description: How to add or update translatable strings with Lingui (Trans / t macros) and manage the en/fr PO catalogs. Use this when adding user-facing text, translating the frontend, or working with i18n/locales.
---

# Internationalization (Lingui)

Setup in `src/i18n/` (instance, `Locale = 'en' | 'fr'`, `loadLocale`), catalogs in
`src/i18n/locales/{en,fr}/messages.po`, config in `lingui.config.ts`. The `@lingui/vite-plugin`
compiles catalogs at build/dev time.

## Marking strings

- **In JSX** → `<Trans>`:
  ```tsx
  import { Trans } from '@lingui/macro'
  <Button><Trans>Create Discussion</Trans></Button>
  ```
- **In expressions / props / non-JSX** → the `t` macro (tagged template):
  ```tsx
  import { t } from '@lingui/macro'
  addNotification({ type: 'success', title: t`Discussion Created` })
  <ContentLayout title={t`Things`} />
  ```

Interpolate with template values: `` t`Hello ${name}` `` / `<Trans>Hello {name}</Trans>`.

## Workflow

```bash
bun run i18n:extract   # scan macros → update en/fr messages.po
# translate the new entries in src/i18n/locales/fr/messages.po
bun run i18n:compile   # compile catalogs for runtime
bun run i18n:check     # extract --clean + compile --strict (CI guard)
```

## Rules

- **Every user-facing string** (labels, buttons, titles, toasts, nav items) goes through `<Trans>` or `` t`...` ``. Don't ship bare literals.
- **No string concatenation** as a translation unit — use interpolation so the whole sentence is one catalog entry.
- **Sidebar/nav labels** in `dashboard-layout.tsx` use `` t`...` `` (see its `navigation` array).
- After adding strings, run `i18n:extract` and add the French translation; don't leave `fr` empty (CI `i18n:check` is strict).
- `defaultLocale` is `en`; add a new locale by extending the `Locale` union + `localeLabels` in `src/i18n/index.ts` and `lingui.config.ts`.
