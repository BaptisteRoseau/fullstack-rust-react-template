# 06 – Tooling: styles, state, i18n, tests

← [Back to overview](README.md)

---

## Global styles and tokens

```
src/css/
├── main.scss           # aggregator imported once by main.tsx
├── _reset.scss         # normalise / box-sizing
├── _variables.scss     # SCSS design tokens (compile-time)
├── _themes.scss        # CSS custom properties (runtime, light/dark)
├── _mixins.scss        # focus-ring, visually-hidden, media queries
└── _typography.scss    # base element styles
```

The split matters:

- **SCSS variables** for values that never change at runtime — spacing scale, radii, font sizes,
  breakpoints, durations. They compile away and cost zero bytes.
- **CSS custom properties** for values that change at runtime — every colour, because of theming.

```scss
// src/css/_variables.scss
$space-1: 0.25rem;
$space-2: 0.5rem;
$space-3: 0.75rem;
$space-4: 1rem;
$space-6: 1.5rem;

$radius-sm: 2px;
$radius-md: 6px;
$radius-full: 9999px;

$font-size-sm: 0.875rem;
$font-size-base: 1rem;
$font-size-lg: 1.125rem;

$breakpoint-sm: 640px;
$breakpoint-md: 768px;
$breakpoint-lg: 1024px;

$duration-fast: 120ms;
```

```scss
// src/css/_themes.scss
:root {
    --color-primary: #2f6feb;
    --color-primary-hover: #2559c4;
    --color-on-primary: #ffffff;
    --color-text: #16181d;
    --color-surface: #ffffff;
    --color-border: #d8dbe0;
    --color-danger: #d13438;
    --color-on-danger: #ffffff;
}

[data-theme='dark'] {
    --color-text: #e8eaed;
    --color-surface: #16181d;
    --color-border: #33373d;
}
```

```scss
// src/css/_mixins.scss
@mixin focus-ring {
    &:focus-visible {
        outline: 2px solid var(--color-primary);
        outline-offset: 2px;
    }
}

@mixin visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
}

@mixin media-up($breakpoint) {
    @media (min-width: $breakpoint) { @content; }
}
```

### Mobile first, always

Every breakpoint is a `min-width`. Base rules describe the **narrowest** viewport — one column,
stacked, full width — and each `@include media-up(...)` step adds what a wider screen affords.
`media-up` is the only media-query mixin on purpose: a `max-width` query means a desktop default
is being undone, which is the pattern this rule exists to prevent.

```scss
.nav {
    display: flex;
    flex-direction: row;          // phone: a row of tabs

    @include media-up($breakpoint-md) {
        flex-direction: column;   // desktop: a sidebar
        position: sticky;
    }
}
```

### One container per section

`container` is the page's horizontal frame: full width with a gutter on a phone, then a
`max-width` that steps up through the ladder so the content stops stretching and sits between side
margins instead.

```scss
@mixin container {
    width: 100%;
    margin-inline: auto;
    padding-inline: $space-4;

    @include media-up($breakpoint-xs) { max-width: $breakpoint-xs; }
    // … one step per breakpoint, up to $breakpoint-2xl
}
```

**Every top-level section of every page includes it** — the hero, the feature grid, the account
grid, the not-found panel. Include it on the section itself, not on a wrapper around the whole
page: full-bleed backgrounds, sticky headers and borders then span the viewport while their
contents stay aligned with every other section. `AppHeader` and `AppFooter` show the shape — a
full-width `.header` with a contained `.inner`.

It touches the inline axis only (`margin-inline`, `padding-inline`), so a section keeps its own
`padding-block` no matter where the include sits in the rule. Never nest one container inside
another: the gutter would be applied twice. A page rendered inside a contained section — anything
under `ContentLayout` — is already framed.

### Making tokens importable

Put `src/css` on the Sass load path so every module can `@use 'variables' as *` without relative
chains:

```ts
// vite.config.ts
export default defineConfig({
    resolve: {
        alias: { '@': path.resolve(__dirname, 'src') },
    },
    css: {
        preprocessorOptions: {
            scss: {
                loadPaths: [path.resolve(__dirname, 'src/css')],
            },
        },
    },
});
```

`@use` is scoped per file, so importing tokens in twenty modules does **not** duplicate CSS —
variables and mixins emit nothing on their own. Only `main.scss` may contain actual global rules.

---

## Global state

Zustand, for state that is genuinely app-wide and not server data. In practice: notifications and
theme. Everything else is SWR (server state), context (scoped state) or `useState` (local).

```ts
// src/stores/notifications.ts
import { nanoid } from 'nanoid';
import { create } from 'zustand';

export type Notification = {
    id: string;
    type: 'info' | 'success' | 'warning' | 'error';
    title: string;
    message?: string;
};

type NotificationsStore = {
    notifications: Notification[];
    addNotification: (notification: Omit<Notification, 'id'>) => void;
    dismissNotification: (id: string) => void;
};

export const useNotifications = create<NotificationsStore>(set => ({
    notifications: [],
    addNotification: notification =>
        set(state => ({ notifications: [...state.notifications, { id: nanoid(), ...notification }] })),
    dismissNotification: id =>
        set(state => ({ notifications: state.notifications.filter(n => n.id !== id) })),
}));
```

Before adding a store, check it is not one of these mistakes: caching an API response (use SWR),
sharing state between a parent and child (props), or state used by one subtree (context).

---

## i18n (Lingui)

```
src/i18n/
├── index.ts                # i18n instance + activateLocale
├── locales/
│   ├── en.po
│   └── fr.po
└── README.md
```

Macros only — never a raw string in JSX:

```tsx
import { Trans, useLingui } from '@lingui/react/macro';

export function Greeting({ name }: { name: string }) {
    const { t } = useLingui();

    return (
        <>
            <h1><Trans>Welcome back, {name}</Trans></h1>
            <input aria-label={t`Search users`} />
        </>
    );
}
```

`<Trans>` for JSX content, `` t`` `` for attributes and non-JSX strings. The Lingui Vite plugin
compiles macros through SWC — **no Babel is involved** anywhere in this toolchain.

```bash
bun run i18n:extract    # scan sources → update en.po / fr.po
bun run i18n:compile    # PO → runtime catalogs
bun run i18n:check      # CI gate: extraction is clean and complete
```

---

## Testing

Three levels, each with a different test double.

| Level | Runner | Location | Doubles |
|---|---|---|---|
| Unit — primitive, hook, util | Vitest | next to source | none |
| Integration — component, page | Vitest + Testing Library | next to source | `vi.mock('@/api/hooks/useApiXxx')` |
| Domain fetcher | Vitest | `api/domains/<domain>/<domain>.test.ts` | MSW |
| API hook | Vitest | `api/hooks/useApiXxx/*.test.ts` | MSW |
| End-to-end | Playwright | `e2e/` | MSW dev server or real backend |

### Test doubles: which one

**Automocked api hooks (default for component and page tests).** `vi.mock('@/api/hooks/useApiXxx')`
plus `vi.mocked(useApiXxx).mockReturnValue(...)` lets a test state "the request is loading" or "the
request failed" in one line instead of orchestrating a network response, and needs no
hand-maintained double. Use it whenever the subject is the UI.

**MSW.** Use it when the subject is the transport: domain fetchers, api hooks, retry and
error-mapping behaviour, tests spanning several domains, plus the dev mock server and e2e. It is
also what `mock-server.ts` runs.

Do not mix both for one subject — pick the level you are testing.

### `src/test-utils/`

```
src/test-utils/
├── render.tsx              # RTL render wrapped in the app's provider tree
├── renderAppAtRoute.tsx    # render the whole router at a given path
├── wrappers.tsx            # SwrWrapper for renderHook
├── server.ts               # MSW node server
├── enableMocking.ts        # MSW browser worker (used by main.tsx)
├── setup-tests.ts          # global setup: jest-dom, server lifecycle, cleanup
├── mocks/
│   ├── handlers/           # one file per domain
│   ├── db.ts               # in-memory database
│   └── browser.ts
└── fixtures/
    ├── auth.ts             # buildCurrentUser() domain + buildGetMeResponse() wire
    └── apiKeys.ts          # buildApiKey() domain + buildGetApiKeyResponse() wire
```

```tsx
// src/test-utils/render.tsx
import { render as rtlRender, type RenderOptions } from '@testing-library/react';
import { MemoryRouter } from 'react-router';
import { SWRConfig } from 'swr';

import { Context } from '@/Context';

export function render(ui: React.ReactElement, { route = '/', ...options }: RenderOptions & { route?: string } = {}) {
    function Wrapper({ children }: { children: React.ReactNode }) {
        return (
            // A fresh Map per render keeps the SWR cache from leaking across tests.
            <SWRConfig value={{ provider: () => new Map(), dedupingInterval: 0 }}>
                <Context>
                    <MemoryRouter initialEntries={[route]}>{children}</MemoryRouter>
                </Context>
            </SWRConfig>
        );
    }

    return rtlRender(ui, { wrapper: Wrapper, ...options });
}
```

Fixtures are **builders**, not frozen objects, so a test can state only what it cares about:

```ts
// src/test-utils/fixtures/users.ts
import { randEmail, randFirstName, randUuid } from '@ngneat/falso';
import type { User } from '@/api/domains/users';

export function buildUser(overrides: Partial<User> = {}): User {
    return {
        id: randUuid(),
        email: randEmail(),
        firstName: randFirstName(),
        lastName: 'Doe',
        role: 'user',
        createdAt: new Date().toISOString(),
        ...overrides,
    };
}
```

### Assertions

Per the project standard, every `assert`/`expect` carries a message showing the offending values:

```ts
expect(result.totalCount, `expected 3 users, got ${result.totalCount}`).toBe(3);
```

### CSS Modules in Vitest

Vitest does **not** process CSS by default, so `styles.button` is `undefined` in tests and class
assertions silently pass or fail for the wrong reason. Two consequences:

1. Assert on roles, labels and text — never on class names. This is the right habit regardless.
2. If you genuinely need class names (rare — a visual-state test), enable readable ones:

```ts
// vite.config.ts, test section
test: {
    css: { modules: { classNameStrategy: 'non-scoped' } },
}
```

### Snapshots

Reserved for pure presentational primitives with many variants, stored in `__snapshots__/` next to
the test. Never snapshot a component that fetches data — the snapshot becomes a diff of your mock.

---

## Storybook

```
src/stories/
├── Introduction.mdx
├── tokens.mdx              # colour / spacing scale documentation
└── decorators/
    ├── CenteredStory.tsx
    └── ThemeDecorator.tsx  # toggles [data-theme] for light/dark
```

Stories are colocated with components; only global config lives here. `.storybook/preview.ts`
imports `src/css/main.scss` so stories render with the real tokens.

Titles: `Design System/<Component>` for primitives, `Components/<Component>` for shared components.
Pages do not get stories — they get tests.

---

## End-to-end (Playwright)

```
e2e/
├── auth.spec.ts
├── users.spec.ts
├── settings.spec.ts
└── utils/
    ├── a11yCheck.ts
    └── login.ts
```

```ts
// e2e/users.spec.ts
import { expect, test } from '@playwright/test';

import { a11yCheck } from './utils/a11yCheck';
import { login } from './utils/login';

test.describe('Users', () => {
    test.beforeEach(async ({ page }) => {
        await login(page);
    });

    test('lists users', async ({ page }) => {
        await page.goto('/users');
        await expect(page.getByRole('heading', { name: 'Users' })).toBeVisible();
        await a11yCheck(page);
    });
});
```

Locate by role and accessible name, as in unit tests. Never by CSS class — SCSS Module hashes
change on every build.

---

## Linting

ESLint keeps the layering honest where it can. Beyond the standard React/a11y/import rules, two
configurations carry architectural meaning:

**File naming** — `eslint-plugin-check-file`, updated for the PascalCase component convention:

```js
'check-file/folder-naming-convention': ['error', {
    'src/design-system/*/': 'PASCAL_CASE',
    'src/components/**/': 'PASCAL_CASE',
    'src/pages/*/': 'PASCAL_CASE',
}],
'check-file/filename-naming-convention': ['error', {
    'src/**/*.module.scss': 'KEBAB_CASE',
    'src/{hooks,utils,types,api}/**/*.{ts,tsx}': 'CAMEL_CASE',
}],
```

**Layer boundaries** — `no-restricted-imports`, since layer-first gives the linter less to work
with than feature-first did:

```js
{
    files: ['src/design-system/**'],
    rules: {
        'no-restricted-imports': ['error', {
            patterns: [
                { group: ['@/api/*', '@/api/domains/*', '@/contexts/*', '@/components/*', '@/pages/*'],
                  message: 'The design system must stay domain-agnostic.' },
            ],
        }],
    },
},
{
    files: ['src/pages/*/**'],
    rules: {
        'no-restricted-imports': ['error', {
            patterns: [{ group: ['@/pages/*/*'], message: 'Pages must not import each other. Move shared code to src/components.' }],
        }],
    },
},
```

These two rules are what stop the architecture eroding. Without them the layering is only a
convention.
