# End-to-end tests

Playwright specs, one file per journey. `playwright.config.ts` starts the Vite dev server and the
MSW-backed mock API (`npm run run-mock-server`) itself, so `npm run test:e2e` needs nothing running.

| Spec | Journey |
|---|---|
| `auth.spec.ts` | register, log in, log out, route guard, redirect back after login |
| `home.spec.ts` | hero, features, footer, not-found |
| `user.spec.ts` | profile edit and API key create/list/revoke |
| `i18n.spec.ts` | locale switch and persistence |
| `responsive.spec.ts` | mobile, tablet and desktop layout |

## Rules

- Import `test` and `expect` from `./utils/fixtures`, never from `@playwright/test`. That fixture
  resets the mock database (`POST /api/__reset`) before every test, which is what makes the suite
  order-independent.
- `utils/session.ts` has `login`, `register`, `logout` and the signed-in/out assertions.
- Locate by role and accessible name. Never by CSS class — SCSS Module hashes change every build.
- Every `expect` that could fail ambiguously carries a message showing the offending value.

## Running against system Chrome

Playwright's browser CDN may be blocked. If `npx playwright install` fails, point the run at an
installed Chrome:

```bash
npx playwright test --config=playwright.local.config.ts
```
