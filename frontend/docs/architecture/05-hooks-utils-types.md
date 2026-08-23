# 05 – Hooks, utils & types

← [Back to overview](README.md)

Three cross-cutting modules plus the contexts and constants folders. Everything here sits at the
bottom of the dependency graph: it may not import from `components/`, `pages/` or `layouts/`.

---

## `src/hooks/`

Reusable React hooks with no domain knowledge. A hook that calls an API service is not a shared
hook — it belongs in `api/hooks/` (if it wraps a request) or in the page's `hooks/` folder.

```
src/hooks/
├── useBooleanState/
│   ├── useBooleanState.ts
│   ├── useBooleanState.test.ts
│   └── index.ts
├── useCopyToClipboard/
│   ├── useCopyToClipboard.ts
│   ├── useCopyToClipboard.test.ts
│   └── index.ts
├── useLocalStorage/
│   ├── useLocalStorage.ts
│   ├── useLocalStorage.test.ts
│   └── index.ts
└── __mocks__/
    └── useIntersectionObserver.ts   # jsdom has no IntersectionObserver
```

One folder per hook — implementation, test and barrel — so consumers keep importing
`@/hooks/useBooleanState` whatever the folder holds. `bun run generate hook` scaffolds the three
files; never add them by hand.

```ts
// src/hooks/useBooleanState/useBooleanState.ts
import { useCallback, useState } from 'react';

export function useBooleanState(initialValue = false) {
    const [value, setValue] = useState(initialValue);

    const setTrue = useCallback(() => setValue(true), []);
    const setFalse = useCallback(() => setValue(false), []);
    const toggle = useCallback(() => setValue(current => !current), []);

    return { value, setTrue, setFalse, toggle };
}
```

Conventions:

- Name is `useXxx`; the folder, the file and the hook all carry that same name.
- Return an object when there is more than one value; a tuple only for `[value, setValue]` pairs.
- Memoise returned callbacks with `useCallback` — a hook's consumers put these in dependency
  arrays.
- The barrel re-exports the hook and its types, nothing else. Import from `@/hooks/useXxx`, never
  from `@/hooks/useXxx/useXxx`.

### Testing a hook

`renderHook` and `act` from Testing Library, one behaviour per `it`, and the project's
message-carrying assertions. Shared hooks own no domain knowledge, so they need no MSW and no
manual `__mocks__` — a hook that would need them belongs in `api/hooks/` instead. Only reach for
`SwrWrapper` from `@/test-utils/wrappers` when the hook genuinely sits on SWR.

Stub the browser APIs jsdom lacks in the test file itself, and undo the stub in `afterEach`;
promote a stub into `src/test-utils/` only once a second hook needs it. `setup-tests.ts` clears
mocks and the mock database but **not** `localStorage` or global stubs, so a hook touching either
cleans up after itself:

```ts
beforeEach(() => {
    vi.useFakeTimers();
    vi.stubGlobal('navigator', { clipboard: { writeText } });
});

afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
});
```

Cover the initial value, each transition, the memoisation contract the third convention above
promises, and the failure path — a rejected clipboard write, a missing stored key.

---

## `src/utils/`

Pure functions and non-React helpers. No JSX, no hooks, no imports from `api/`.

```
src/utils/
├── createContext.tsx       # typed createContext factory
├── date.ts                 # dayjs formatting helpers
├── format.ts               # numbers, bytes, currency
├── strings.ts              # truncate, slugify, initials
├── arrays.ts               # groupBy, uniqueBy, sortBy
├── url.ts                  # query-string helpers
└── assert.ts               # invariant / assertNever
```

Every file exports named functions only — no default exports, no god-object `helpers.ts`. If a
helper is used by exactly one module, keep it in that module's `utils.ts` instead.

### Typed context factory

Removes the `| undefined` dance from every context and gives a real error when a provider is
missing.

```tsx
// src/utils/createContext.tsx
import { createContext as reactCreateContext, useContext as reactUseContext } from 'react';

export function createContext<T>(displayName: string) {
    const Context = reactCreateContext<T | undefined>(undefined);
    Context.displayName = displayName;

    function useContext(): T {
        const value = reactUseContext(Context);
        if (value === undefined) {
            throw new Error(`use${displayName} must be used inside <${displayName}Provider>`);
        }
        return value;
    }

    return [Context.Provider, useContext] as const;
}
```

### `assertNever`

Makes an unhandled union member a compile error rather than a silent fallthrough:

```ts
export function assertNever(value: never): never {
    throw new Error(`Unhandled case: ${JSON.stringify(value)}`);
}
```

---

## `src/contexts/`

One folder per context, each with the definition, the provider and the consumer hook separated so
that importing the hook does not pull in the provider's dependencies.

```
src/contexts/
├── auth/
│   ├── AuthContext.ts
│   ├── AuthContextProvider.tsx
│   ├── useAuth.ts
│   ├── types.ts
│   └── index.ts
└── theme/
    ├── ThemeContext.ts
    ├── ThemeContextProvider.tsx
    ├── useTheme.ts
    └── index.ts
```

```tsx
// src/contexts/auth/AuthContext.ts
import { createContext } from '@/utils/createContext';
import type { AuthContextValue } from './types';

export const [AuthProvider, useAuth] = createContext<AuthContextValue>('Auth');
```

Contexts hold **scoped UI or session state**. Server data belongs in SWR — a context that caches an
API response is duplicating the cache you already have.

---

## `src/constants/`

App-wide constants that are not routes (those live in `router/constants.ts`) and not module-local.

```
src/constants/
├── index.ts
├── pagination.ts       # DEFAULT_PAGE_SIZE, PAGE_SIZE_OPTIONS
├── dates.ts            # DATE_FORMAT, DATETIME_FORMAT
└── storage.ts          # localStorage keys
```

`SCREAMING_SNAKE_CASE` for values, `as const` for object literals so their types stay narrow.

---

## `src/types/`

TypeScript types shared across layers. Domain types describing an API payload live in
[`api/domains/<domain>/types.ts`](01-api.md) next to their fetchers — this folder is for everything else.

```
src/types/
├── index.ts            # barrel
├── common.ts           # Nullable<T>, DeepPartial<T>, AsyncState<T>
├── pagination.ts       # PaginatedResponse<T>, PageParams
├── declarations.d.ts   # module augmentations (*.svg?react, *.module.scss)
└── env.d.ts            # ImportMetaEnv
```

```ts
// src/types/common.ts
export type Nullable<T> = T | null;

export type PaginatedResponse<T> = {
    results: T[];
    totalCount: number;
};
```

```ts
// src/types/declarations.d.ts
declare module '*.svg?react' {
    const ReactComponent: React.FC<React.SVGProps<SVGSVGElement>>;
    export default ReactComponent;
}

declare module '*.module.scss' {
    const classes: Readonly<Record<string, string>>;
    export default classes;
}
```

The `*.module.scss` declaration is what makes `styles.button` type-check. Without it every
stylesheet import is an error under `strict`.

---

## Rules of thumb

| You have… | It goes in… |
|---|---|
| A pure function used by 2+ modules | `utils/<topic>.ts` |
| A pure function used by 1 module | that module's `utils.ts` |
| A hook with no domain knowledge | `hooks/` |
| A hook that wraps a request | `api/hooks/useApiXxx/` |
| A hook used by one page | `pages/<Page>/hooks/` |
| A type describing an API payload | `api/domains/<domain>/types.ts` |
| A generic type helper | `types/common.ts` |
| Session or scoped UI state | `contexts/<name>/` |
| Genuinely app-wide UI state | the Zustand store — see [06](06-tooling.md#global-state) |
