# e2e

Playwright specs, one file per journey. `playwright.config.ts` starts the Vite dev server and the
MSW-backed mock API (`bun run run-mock-server`) itself, so `bun run test:e2e` needs nothing running
beforehand.

```txt
e2e/
├── auth.spec.ts            # register, log in, log out, route guard, redirect back after login
├── home.spec.ts            # hero, features, footer, not-found
├── user.spec.ts            # profile edit and API key create/list/revoke
├── i18n.spec.ts            # locale switch and persistence
├── responsive.spec.ts      # mobile, tablet and desktop layout
└── utils/
    ├── fixtures.ts         # `test`/`expect` — resets the mock database before every test
    └── session.ts          # login, register, logout helpers and signed-in/out assertions
```

Specs import `test` and `expect` from `./utils/fixtures`, never from `@playwright/test` — that
fixture is what keeps the suite order-independent. Locate elements by role and accessible name,
never by CSS class: SCSS Module hashes change every build.

Playwright's browser CDN may be blocked in some environments. If `npx playwright install` fails,
point the run at an installed Chrome instead: `npx playwright test --config=playwright.local.config.ts`.

## Skills

- [frontend-testing](../../.claude/skills/frontend-testing/SKILL.md)
