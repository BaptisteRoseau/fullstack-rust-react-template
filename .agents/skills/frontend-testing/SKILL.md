---
name: frontend-testing
description: Use when adding or updating a Vitest, Testing Library or Playwright test for the frontend.
---

# Testing

| Level | Runner | Location | Doubles |
| --- | --- | --- | --- |
| Unit — primitive, hook, util | Vitest | next to source | none |
| Integration — component, page | Vitest + Testing Library | next to source | `vi.mock('@/api/hooks/useApiXxx')` |
| Domain fetcher | Vitest | `src/api/domains/<domain>/<domain>.test.ts` | MSW |
| API hook | Vitest | `src/api/hooks/useApiXxx/*.test.ts` | MSW |
| End-to-end | Playwright | `e2e/` | `mock-server.ts` |

Do not create a test file next to a new component, page, API service or hook by hand — the Plop
generator that scaffolds the folder writes it (`bun run generate <component|page|api|hook> …`, see
Skill(frontend-architecture)). Fill in the generated file. Only e2e specs under `e2e/` are written
from scratch.

## 1. Assert with a message

Every assertion carries a message showing the offending value:

```ts
expect(
    result.current.data?.length,
    `expected 1 key, got ${result.current.data?.length} (error: ${result.current.error})`,
).toBe(1)
```

Query by **role, label and text** — never by class name. SCSS Module hashes are not a contract, and
Vitest does not process CSS, so `styles.button` is `undefined` in tests.

## 2. Render with the app's helpers

```txt
src/test-utils/
├── render.tsx              # RTL render inside the app provider tree + MemoryRouter
├── renderAppAtRoute.tsx    # render the router at a given path
├── wrappers.tsx            # SwrWrapper for renderHook
└── setup-tests.ts          # jest-dom, MSW lifecycle, locale, db reset
```

Use `render` from [`@/test-utils/render`](../../../frontend/src/test-utils/render.tsx) for anything
that needs i18n, SWR or the router. Plain Testing Library `render` is fine for design-system
primitives. Each render gets a fresh SWR cache (`provider: () => new Map()`), so results never leak
between tests.

## 3. Write a hook test

`src/hooks/useXxx/useXxx.test.ts`, written with `renderHook` and `act`. Shared hooks own no domain
knowledge, so they need no MSW and no module mock; only reach for
[`SwrWrapper`](../../../frontend/src/test-utils/wrappers.tsx) when the hook sits on SWR. Stub the
browser APIs jsdom lacks in the test file itself and undo them in `afterEach` —
`setup-tests.ts` clears mocks and the mock database but **not** `localStorage` or global stubs.

Cover the initial value, each transition, the `useCallback` identity the hook promises its
consumers, and the failure path.

## 4. Write a component or page test

Mock the service, assert the rendering. See
[src/components/layout/AppHeader/AppHeader.test.tsx](../../../frontend/src/components/layout/AppHeader/AppHeader.test.tsx):
`vi.mock('@/api/hooks/useApiCurrentUser')`, then `vi.mocked(...).mockReturnValue(...)` with the
full SWR result shape — the mocked return must include `isValidating`, since SWR's type requires
it. Test the states that break in production: loading, error, empty, and populated.

## 5. Write a service test

Go through the transport, backed by MSW — see Skill(frontend-api) and
[src/api/domains/apiKeys/apiKeys.test.ts](../../../frontend/src/api/domains/apiKeys/apiKeys.test.ts).

## 6. Add an e2e spec, for a critical journey

Read [e2e.md](./e2e.md).

## Storybook

Every design-system primitive has a story; the preview supplies i18n, a router and a light/dark
toolbar toggle. Pages do not get stories — they get tests.

```bash
bun run storybook
```

## Checklist

```bash
bun run test
```

- [ ] Loading, error, empty and populated states are all covered for a component or page test.
- [ ] Every assertion that could fail ambiguously carries a message with the offending value.
