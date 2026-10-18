---
name: frontend-accessibility-checking
description: Use when checking or fixing accessibility (a11y, WCAG, ARIA, contrast, light or dark theme) of frontend pages, or before validating a task that changed files under frontend/src.
---

# Check the frontend accessibility

`oxaria` loads a page of the running frontend, runs its JavaScript, and reports the WCAG 2.2, ARIA
and ACT rules it breaks. It checks what a user gets, so the frontend must be running and serve
your latest edits. Pick the mode that matches the scope of the change:

| Changed | Mode |
| --- | --- |
| One page folder under `src/pages/` | [A single page](#2-a-single-page) |
| A component, a form or a hook | [A list of pages](#3-a-list-of-pages) |
| A layout, `src/design-system/`, `src/css/`, or before a release | [A full website sweep](#4-a-full-website-sweep) |
| A colour token, or any change to `src/css/_themes.scss` | [Both themes](#5-check-both-themes) |

## Prerequisite

Consider `oxaria` installed, but if the command is not found follow [install.md](./install.md),
then resume your task. Do not read it otherwise. Run `oxaria --help` for the options not covered
here.

## 1. Start the frontend on the mock API

Check against the mock API, the same setup `bun run test:e2e` uses. Its data is fixed and its
session needs no real login. From `frontend/`, start both servers in the background:

```bash
bun run run-mock-server
VITE_APP_API_URL=http://localhost:8081 VITE_APP_URL=http://localhost:3000 bun run dev
```

The base URL is then `http://localhost:3000`. If a dev server already runs on port 3000, check
that it uses the mock API before reusing it; never check against the release container, which
serves a build made before your edits.

A page path comes from `PATHS` in
[src/router/constants.ts](../../../frontend/src/router/constants.ts). The `*` entry is the 404
page: check it with any unknown path, such as `/does-not-exist`.

### Signed-in pages

The routes wrapped in `ProtectedRoute` in
[src/router/routes.tsx](../../../frontend/src/router/routes.tsx) (`/user`, `/user/api-keys`) need a
session. Without one they redirect to `/auth/login`, and `oxaria` silently checks the login page
instead.

Pass the session on every `oxaria` call with `-H`, which works like `curl`'s and can be repeated.
The mock API accepts a fixed cookie, which is not a secret:

```bash
oxaria -H "Cookie: mock_session=active" "$BASE_URL/user"
```

Only when the user asks to check against the real backend, the session is the `access_token` and
`refresh_token` httpOnly cookies instead:

- Ask the user for the two values. They copy them from the browser developer tools
  (Application, then Cookies) after logging in.
- Keep them in shell variables only. Never write them in a file, a commit, a test or your report.
- The access token is short-lived. When a signed-in page reports the findings of the login page,
  ask for fresh values.

## 2. A single page

```bash
oxaria -H "Cookie: mock_session=active" "$BASE_URL/user/api-keys"
```

Fix the findings, then run the same command again. Repeat until it exits 0.

## 3. A list of pages

List the pages that render your changes, then check them in one call:

```bash
.claude/skills/frontend-accessibility-checking/scripts/affected_pages.sh --base "$BASE_URL"
.claude/skills/frontend-accessibility-checking/scripts/check_pages.sh \
    -H "Cookie: mock_session=active" \
    $(.claude/skills/frontend-accessibility-checking/scripts/affected_pages.sh --base "$BASE_URL")
```

`affected_pages.sh` follows the imports from the files changed since `HEAD` up to the page
folders, and maps each page to its URLs through the router. Pass files as arguments to start from
other ones. A change to `src/css/`, `src/i18n/`, `src/router/`, `src/layouts/` or the app entry
files lists every page.

`check_pages.sh` runs `oxaria` once per URL, prints the findings of the failing ones, and ends with
the list of URLs that still fail. Every `-H` is passed to each call. Fix, rerun with only the
failing URLs, and once more with the full list at the end.

## 4. A full website sweep

```bash
oxaria --full-site-scan -H "Cookie: mock_session=active" "$BASE_URL/"
```

The crawl follows the links of each page, so it reaches only linked pages: the user pages are
linked from the user menu, which shows only with a session. Without the cookie, the sweep misses
them. Each finding starts with the path of its page, such as `[ERROR] WCAG 2.2 H57 /user: ...`.

A sweep can return many findings. Group them by rule before fixing: a finding repeated on every
page comes from a layout or a shared component, so fix it once there.

## 5. Check both themes

The theme is the `data-theme` attribute of `<html>`, set by
[src/stores/theme.ts](../../../frontend/src/stores/theme.ts) from the `app-theme` local storage
key, or from the system `prefers-color-scheme` when the user picked "system". Each theme is one
block of colour tokens in [src/css/_themes.scss](../../../frontend/src/css/_themes.scss).

`oxaria` starts with empty storage and no dark preference, so every mode above checks the **light
theme only**. The dark theme is covered by checking its tokens:

```bash
.claude/skills/frontend-accessibility-checking/scripts/check_theme_contrast.sh
```

It checks, in both blocks, every text pair against 4.5:1 and every control edge or focus outline
against 3:1. Run it after any change to `_themes.scss`, and add a pair to its lists when you
introduce a token that carries text or draws the edge of a control.

- Fix a contrast finding in the token of the theme that fails, then rerun the script: a light
  token fix must not break its dark counterpart, and the reverse.
- `--color-border` is both the decorative border of cards and tables and the only edge of the
  inputs. Do not darken it to fix an input edge: add a separate token for control edges, use it in
  `src/design-system/inputs/`, and move the pair in the script to the new token.
- Never hard-code a colour in a module. It escapes the theme switch and stays the same in both
  themes, so one of them is likely to fail.

## 6. Fix the findings

Every element finding ends with a CSS-like path to the element, and every finding has a `help`
line with the fix.

- Fix the code, never the check: do not raise `--contrast-threshold` or
  `--target-size-threshold`, and do not drop a standard from `--standards`.
- A colour finding is fixed in the design tokens, not with a one-off colour in a module —
  Skill(frontend-design-system).
- New visible or screen-reader text, such as an `aria-label` or a skip link, goes through Lingui —
  Skill(frontend-i18n).

## Checklist

```bash
.claude/skills/frontend-accessibility-checking/scripts/check_pages.sh -H "Cookie: mock_session=active" \
    $(.claude/skills/frontend-accessibility-checking/scripts/affected_pages.sh --base http://localhost:3000)
.claude/skills/frontend-accessibility-checking/scripts/check_theme_contrast.sh
cd frontend && bun run i18n:check && bun run lint && bun run test
```

- [ ] Every page the change renders was checked with the session cookie.
- [ ] No finding was silenced by a threshold, a `--standards` change or a removed element.
