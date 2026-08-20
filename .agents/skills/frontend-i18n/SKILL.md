---
name: frontend-i18n
description: How to add or update translatable strings with Lingui macros and keep the en/fr PO catalogs complete. Use this when adding user-facing text, translating the frontend, or fixing an i18n:check failure.
---

# i18n (Lingui)

```
src/i18n/
├── index.ts                # i18n instance, locales, loadLocale
└── locales/{en,fr}/messages.po
```

**Never write a raw user-facing string in JSX.** Every one goes through a macro.

## Macros

```tsx
import { Trans, useLingui } from '@lingui/react/macro'

export function Greeting({ name }: { name: string }) {
    const { t } = useLingui()

    return (
        <>
            <h1><Trans>Welcome back, {name}</Trans></h1>
            <input aria-label={t`Search users`} />
        </>
    )
}
```

- `<Trans>` for JSX content.
- `` t`…` `` for attributes, `aria-label`s, titles, notification text and anything not JSX.
- `t` comes from `useLingui()` — a hook, so it re-renders when the locale changes. Do not import
  the standalone `t` macro; a string captured once will not update on a locale switch.

Interpolation works in both: `` t`Revoke ${apiKey.name}` `` and
`<Trans>Copy {apiKey.name} now</Trans>`.

## Workflow

```bash
npm run i18n:extract    # scan sources → update en.po / fr.po
npm run i18n:compile    # PO → runtime catalogs
npm run i18n:check      # CI gate: extraction is clean and complete
```

After adding any string: extract, **fill in the French translation**, then compile.
`i18n:check` compiles with `--strict`, so a single missing `msgstr` fails CI.

Edit `src/i18n/locales/fr/messages.po` by hand — find the `msgid` and write the `msgstr`:

```po
msgid "Save changes"
msgstr "Enregistrer"
```

Use straight apostrophes (`'`) consistently; e2e specs match on the exact string.

## Switching locale

`LocaleSwitcher` (in `src/components/layout/`) calls the locale store, which loads the catalog and
persists the choice. `main.tsx` reads `storedLocale()` before mounting so there is no flash of the
wrong language. Adding a locale means: add it to `locales` in `src/i18n/index.ts`, add its label to
`localeLabels`, create `locales/<code>/messages.po`, run extract.

## Tests

`src/test-utils/setup-tests.ts` activates the default locale before any test runs, so
`screen.getByText('Save changes')` matches the English source string. Assert on the rendered text,
never on the message id.

For e2e, `e2e/i18n.spec.ts` switches locale through the UI and asserts the French strings —
so a translation you change must be updated there too.
