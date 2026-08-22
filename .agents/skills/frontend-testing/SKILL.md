---
name: frontend-testing
description: How to write Vitest + Testing Library tests and Playwright e2e specs for the frontend, including the render helpers, assertion style and responsive/i18n coverage. Use this when adding or updating frontend tests.
---

# Testing

| Level | Runner | Location | Doubles |
|---|---|---|---|
| Unit — primitive, hook, util | Vitest | next to source | none |
| Integration — component, page | Vitest + Testing Library | next to source | manual `__mocks__` |
| Service | Vitest | `src/api/service/*.test.ts` | MSW |
| End-to-end | Playwright | `e2e/` | `mock-server.ts` |

```bash
bun run test          # vitest run
bun run test:watch
bun run test:e2e
```

## Assertion style

Per the project standard, **every assertion carries a message showing the offending value**:

```ts
expect(
    result.current.data?.length,
    `expected 1 key, got ${result.current.data?.length} (error: ${result.current.error})`,
).toBe(1)
```

Query by **role, label and text** — never by class name. SCSS Module hashes are not a contract, and
Vitest does not process CSS, so `styles.button` is `undefined` in tests.

## Render helpers

```
src/test-utils/
├── render.tsx              # RTL render inside the app provider tree + MemoryRouter
├── renderAppAtRoute.tsx    # render the router at a given path
├── wrappers.tsx            # SwrWrapper for renderHook
└── setup-tests.ts          # jest-dom, MSW lifecycle, locale, db reset
```

Use `render` from `@/test-utils/render` for anything that needs i18n, SWR or the router. Plain
Testing Library `render` is fine for design-system primitives. Each render gets a fresh SWR cache
(`provider: () => new Map()`), so results never leak between tests.

## Component and page tests

Mock the service, assert the rendering:

```tsx
import { screen } from '@testing-library/react'

import { useApiKeys } from '@/api/service/apiKeys'
import { buildApiKey } from '@/test-utils/fixtures/apiKeys'
import { render } from '@/test-utils/render'

import { ApiKeys } from './ApiKeys'

vi.mock('@/api/service/apiKeys')

it('lists the api keys', () => {
    vi.mocked(useApiKeys).mockReturnValue({
        data: [buildApiKey({ name: 'CI deploy key' })],
        error: undefined,
        isLoading: false,
        isValidating: false,
        mutate: vi.fn(),
    })

    render(<ApiKeys />)

    expect(
        screen.getByText('CI deploy key'),
        `expected the key row, got: ${document.body.textContent}`,
    ).toBeVisible()
})
```

The mocked return must include `isValidating` — SWR's type requires it.

Test the states that break in production: loading, error, empty, and populated.

## Service tests

Go through the transport, backed by MSW — see the `frontend-api` skill.

## End-to-end

```
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
and accessible name, exactly as in unit tests.

Cover for any new journey:

- the happy path,
- the guard (signed-out access redirects and comes back after login),
- validation failure,
- cancel/undo,
- at least one multi-entity case, so list rendering and per-row actions are exercised.

### Responsive

The CSS is mobile-first. `responsive.spec.ts` drives 375 / 768 / 1440 px and asserts both that
content is reachable and that the page never scrolls horizontally:

```ts
const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
)
expect(overflow, 'the page must not scroll horizontally on mobile').toBeLessThanOrEqual(0)
```

Layout changes across a breakpoint are asserted with bounding boxes (nav above the content on
mobile, beside it on desktop) rather than class names.

### i18n

`i18n.spec.ts` switches locale through the `LocaleSwitcher` and asserts the translated strings and
their persistence across a reload. If you change a French translation, update the spec.

## Storybook

Every design-system primitive has a story; the preview supplies i18n, a router and a light/dark
toolbar toggle. Pages do not get stories — they get tests.

```bash
bun run storybook
bun run storybook:build
```
