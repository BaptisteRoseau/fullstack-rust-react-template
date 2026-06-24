# `i18n/`

Internationalization via **Lingui**.

| File / folder                 | Role                                                                                                            |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `index.ts`                    | The shared `i18n` instance, `Locale = 'en' \| 'fr'`, `defaultLocale`, `localeLabels`, and `loadLocale(locale)`. |
| `locales/{en,fr}/messages.po` | Translation catalogs (gettext PO).                                                                              |

Config is `lingui.config.ts`; the `@lingui/vite-plugin` compiles catalogs.

## Marking strings

- JSX → `<Trans>Create</Trans>`.
- Expressions/props → `` t`Create` `` (tagged template).

## Workflow

```bash
bun run i18n:extract   # scan macros → update PO files
bun run i18n:compile   # compile catalogs for runtime
bun run i18n:check     # strict extract + compile (CI guard)
```

After adding strings, run `i18n:extract` and fill in the French translation (CI `i18n:check` is strict).
Add a locale by extending the `Locale` union + `localeLabels` here and `lingui.config.ts`.

See `.claude/skills/frontend-react-i18n`.
