# 01 – API layer

← [Back to overview](README.md)

`src/api/` owns every HTTP call to the Rust backend. It is split in two so that *what an endpoint
is* stays free of side effects and can be imported anywhere (tests, mocks, e2e), while *how it is
called* lives in one place.

---

## Directory tree

```
src/api/
├── client.ts               # fetch wrapper: base URL, JSON, error normalisation
├── errors.ts               # ApiError type
├── <domain>.ts             # Endpoint paths + request/response types. NO side effects.
├── service/
│   ├── <domain>.ts         # SWR hooks that actually call the endpoints
│   ├── <domain>.test.ts
│   └── __mocks__/
│       └── <domain>.ts     # Manual mock consumed by `vi.mock`
└── utils/
    └── useApiAction.ts     # Generic mutation hook (POST / PUT / PATCH / DELETE)
```

One `<domain>.ts` per backend resource — `users.ts`, `apiKeys.ts`, `health.ts`. The file name in
`service/` always mirrors the declaration file it implements.

---

## The transport (`api/client.ts`)

A single wrapper around `fetch`. It is the only place that knows about the base URL, credentials
and error shape.

```ts
// src/api/client.ts
import { env } from '@/config/env';
import { ApiError } from './errors';

export async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
    const response = await fetch(`${env.API_URL}${path}`, {
        credentials: 'include',
        ...init,
        headers: {
            'Content-Type': 'application/json',
            ...init?.headers,
        },
    });

    if (!response.ok) {
        const body = await response.json().catch(() => null);
        throw new ApiError(`${init?.method ?? 'GET'} ${path} failed`, response.status, body);
    }

    // 204 No Content has no body to parse.
    if (response.status === 204) {
        return undefined as T;
    }

    return (await response.json()) as T;
}
```

```ts
// src/api/errors.ts
export class ApiError extends Error {
    constructor(
        message: string,
        readonly status: number,
        readonly body: unknown,
    ) {
        super(message);
        this.name = 'ApiError';
    }
}
```

`apiFetch` is registered as SWR's global `fetcher` in `src/Context.tsx`, so read hooks never pass
one explicitly:

```tsx
<SWRConfig value={{ fetcher: apiFetch, revalidateOnFocus: false, shouldRetryOnError: false }}>
```

---

## Endpoint declaration (`api/<domain>.ts`)

Constants and types only. Importing this file must never trigger a request.

```ts
// src/api/users.ts
export const USERS_ENDPOINT = '/api/v1/users';
export const userEndpoint = (userId: string) => `${USERS_ENDPOINT}/${userId}`;

export type UserRole = 'admin' | 'user';

export type User = {
    id: string;
    email: string;
    firstName: string;
    lastName: string;
    role: UserRole;
    createdAt: string;
};

export type UserList = {
    results: User[];
    totalCount: number;
};

export type CreateUserBody = Pick<User, 'email' | 'firstName' | 'lastName' | 'role'>;
```

Path builders are plain functions so that call sites, tests and MSW handlers all derive URLs from
the same source.

### When to validate with Zod

Response validation is **opt-in**, not the default. Add a schema when the payload crosses a trust
boundary or is genuinely dynamic; skip it for endpoints the backend types already guarantee. When
you do validate, keep the schema in the declaration file and derive the type from it:

```ts
export const userSchema = z.object({ id: z.string(), email: z.email(), /* … */ });
export type User = z.infer<typeof userSchema>;
```

---

## Service (`api/service/<domain>.ts`)

The hooks components actually call. One hook per operation, named for the operation.

```ts
// src/api/service/users.ts
import useSWR from 'swr';

import { useApiAction } from '@/api/utils/useApiAction';
import {
    USERS_ENDPOINT,
    userEndpoint,
    type CreateUserBody,
    type User,
    type UserList,
} from '@/api/users';

export function useUsers() {
    return useSWR<UserList>(USERS_ENDPOINT);
}

export function useUser(userId: string | undefined) {
    // A null key tells SWR to skip the request entirely.
    return useSWR<User>(userId ? userEndpoint(userId) : null);
}

export function useCreateUser() {
    return useApiAction<CreateUserBody, User>(USERS_ENDPOINT, 'POST');
}

export function useDeleteUser(userId: string) {
    return useApiAction<void, void>(userEndpoint(userId), 'DELETE');
}
```

Rules:

- Components import from `api/service/*`, never from `api/client.ts` directly.
- A hook returns SWR's result object untouched (`{ data, error, isLoading, mutate }`). Do not
  destructure-and-rewrap; callers expect the standard shape.
- Cross-cutting derivations (filtering, sorting, formatting) belong in the component or a `utils/`
  helper, not in the service.

---

## Mutations (`api/utils/useApiAction.ts`)

A thin generic over `useSWRMutation` so every mutation has the same signature.

```ts
// src/api/utils/useApiAction.ts
import useSWRMutation from 'swr/mutation';

import { apiFetch } from '@/api/client';
import type { ApiError } from '@/api/errors';

type Method = 'POST' | 'PUT' | 'PATCH' | 'DELETE';

export function useApiAction<TBody, TResult>(path: string, method: Method = 'POST') {
    return useSWRMutation<TResult, ApiError, string, TBody>(path, (url, { arg }) =>
        apiFetch<TResult>(url, {
            method,
            body: arg === undefined ? undefined : JSON.stringify(arg),
        }),
    );
}
```

Usage — `trigger` rejects on failure, so handle it explicitly rather than swallowing:

```tsx
const { trigger, isMutating } = useCreateUser();
const { mutate } = useSWRConfig();

async function onSubmit(values: CreateUserBody) {
    await trigger(values);
    await mutate(USERS_ENDPOINT); // refresh the list
}
```

### Cache invalidation

SWR keys are the endpoint strings themselves. After a mutation, revalidate with the global
`mutate` from `useSWRConfig()`:

- exact key — `mutate(USERS_ENDPOINT)`
- key prefix — `mutate(key => typeof key === 'string' && key.startsWith(USERS_ENDPOINT))`

Because invalidation is string-matching rather than hierarchical, **always build keys from the path
helpers** in the declaration file. Hand-written URL literals are how stale caches happen.

---

## Manual mock (`api/service/__mocks__/<domain>.ts`)

The default way to keep a component test off the network. Every exported hook of the real module
must be mocked, or importers will crash on an undefined function.

```ts
// src/api/service/__mocks__/users.ts
import { vi } from 'vitest';

export const useUsers = vi.fn().mockReturnValue({
    data: { results: [], totalCount: 0 },
    error: undefined,
    isLoading: false,
    mutate: vi.fn(),
});

export const useUser = vi.fn().mockReturnValue({
    data: undefined,
    error: undefined,
    isLoading: false,
    mutate: vi.fn(),
});

export const useCreateUser = vi.fn().mockReturnValue({
    trigger: vi.fn(),
    isMutating: false,
});

export const useDeleteUser = vi.fn().mockReturnValue({
    trigger: vi.fn(),
    isMutating: false,
});
```

Activate it in a test, then override per case:

```tsx
import { useUsers } from '@/api/service/users';

vi.mock('@/api/service/users');

it('renders the user list', () => {
    vi.mocked(useUsers).mockReturnValue({
        data: { results: [buildUser({ email: 'alice@example.com' })], totalCount: 1 },
        error: undefined,
        isLoading: false,
        mutate: vi.fn(),
    });

    render(<UserList />);

    expect(screen.getByText('alice@example.com'), 'user row should be rendered').toBeVisible();
});
```

Use MSW instead when the test exercises the transport itself, or spans several services — see
[06 – Tooling](06-tooling.md#test-doubles-which-one).

---

## Testing a service (`api/service/<domain>.test.ts`)

Service tests are the one place that *should* go through the network layer, backed by MSW.

```ts
// src/api/service/users.test.ts
import { renderHook, waitFor } from '@testing-library/react';
import { http, HttpResponse } from 'msw';

import { server } from '@/test-utils/server';
import { SwrWrapper } from '@/test-utils/wrappers';
import { USERS_ENDPOINT } from '@/api/users';
import { useUsers } from './users';

it('returns the user list', async () => {
    server.use(
        http.get(`*${USERS_ENDPOINT}`, () =>
            HttpResponse.json({ results: [], totalCount: 0 }),
        ),
    );

    const { result } = renderHook(() => useUsers(), { wrapper: SwrWrapper });

    await waitFor(() =>
        expect(result.current.data, `expected data, got error: ${result.current.error}`)
            .toBeDefined(),
    );
});
```

`SwrWrapper` must disable deduping (`dedupingInterval: 0`) and provide a fresh cache per test,
otherwise results leak between cases.
