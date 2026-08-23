# End-to-end specs

Read this only when adding or changing a Playwright spec for a critical user journey, not for a
unit or component test.

```txt
e2e/
├── auth.spec.ts            # register / login / logout / guard / redirect-back
├── home.spec.ts
├── user.spec.ts            # profile edit + api key CRUD
├── i18n.spec.ts            # locale switch + persistence
├── responsive.spec.ts      # mobile / tablet / desktop
└── utils/
    ├── fixtures.ts         # `test` that resets the mock DB before each test
    └── session.ts          # login/register/logout helpers
```

**Import `test` and `expect` from `./utils/fixtures`, not from `@playwright/test`** — the fixture
resets the mock database before each test, which is what makes the suite order-independent.

`playwright.config.ts` starts both the dev server and the mock API server itself. Locate by role
and accessible name, exactly as in unit tests — see
[e2e/README.md](../../../frontend/e2e/README.md).

Cover for any new journey:

- the happy path,
- the guard (signed-out access redirects and comes back after login),
- validation failure,
- cancel/undo,
- at least one multi-entity case, so list rendering and per-row actions are exercised.

## Responsive

The CSS is mobile-first. `responsive.spec.ts` drives 375 / 768 / 1440 px and asserts both that
content is reachable and that the page never scrolls horizontally. Layout changes across a
breakpoint are asserted with bounding boxes (nav above the content on mobile, beside it on
desktop), never with class names.

## i18n

`i18n.spec.ts` switches locale through the `LocaleSwitcher` and asserts the translated strings and
their persistence across a reload. If you change a French translation, update the spec —
Skill(frontend-i18n).
