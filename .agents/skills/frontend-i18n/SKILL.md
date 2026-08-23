---
name: frontend-i18n
description: Use when adding user-facing text, translating the frontend, or fixing an i18n:check failure.
---

# i18n (Lingui)

```txt
src/i18n/
├── index.ts                # i18n instance, locales, loadLocale
└── locales/{en,fr}/messages.po
```

**Never write a raw user-facing string in JSX.** Every one goes through a macro.

## 1. Wrap the string in a macro

```tsx
const { t } = useLingui()

<h1><Trans>Welcome back, {name}</Trans></h1>
<input aria-label={t`Search users`} />
```

- `<Trans>` for JSX content.
- `` t`…` `` for attributes, `aria-label`s, titles, notification text and anything not JSX.
- `t` comes from `useLingui()` — a hook, so it re-renders when the locale changes. Do not import
  the standalone `t` macro; a string captured once will not update on a locale switch.

Interpolation works in both: `` t`Revoke ${apiKey.name}` `` and
`<Trans>Copy {apiKey.name} now</Trans>`.

## 2. Extract, translate, compile

```bash
bun run i18n:extract    # scan sources → update en.po / fr.po
bun run i18n:compile    # PO → runtime catalogs
```

Edit `src/i18n/locales/fr/messages.po` by hand — find the `msgid` and write the `msgstr`:

```po
msgid "Save changes"
msgstr "Enregistrer"
```

Use straight apostrophes (`'`) consistently; e2e specs match on the exact string.

## 3. Check

```bash
bun run i18n:check
```

CI gate: extraction is clean and every message is translated (`--strict`). A single missing
`msgstr` fails it.

## Adding a locale

Add it to `locales` in `src/i18n/index.ts`, add its label to `localeLabels`, create
`locales/<code>/messages.po`, then run extract. `LocaleSwitcher` (in `src/components/layout/`)
calls the locale store — Skill(frontend-state) — which loads the catalog and persists the choice.

## Tests

`src/test-utils/setup-tests.ts` activates the default locale before any test runs, so
`screen.getByText('Save changes')` matches the English source string. Assert on the rendered text,
never on the message id — Skill(frontend-testing).

For e2e, `e2e/i18n.spec.ts` switches locale through the UI and asserts the French strings — so a
translation you change must be updated there too.

## Checklist

```bash
bun run i18n:check
```

- [ ] The French translation is filled in, not left as the English source string.
