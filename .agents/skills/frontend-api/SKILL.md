---
name: frontend-api
description: How to declare a backend endpoint and its SWR service hooks in src/api (declaration file + service + manual mock + MSW-backed test). Use this when adding or updating a frontend API call, query, mutation or cache invalidation.
---

# Frontend API layer

`src/api/` owns every HTTP call. It is split so that *what an endpoint is* stays side-effect free
and importable anywhere, while *how it is called* lives in one place.

```
src/api/
├── client.ts               # fetch wrapper: base URL, JSON, 401 refresh, error normalisation
├── errors.ts               # ApiError, isApiError, apiErrorMessage
├── <domain>.ts             # endpoint paths + types. NO side effects.
├── service/
│   ├── <domain>.ts         # SWR hooks that call the endpoints
│   ├── <domain>.test.ts    # MSW-backed
│   └── __mocks__/
│       └── <domain>.ts     # manual mock consumed by vi.mock
└── utils/useApiAction.ts   # generic POST/PUT/PATCH/DELETE hook
```

## 1. Declare the endpoint

`src/api/<domain>.ts` — constants and types only. Importing it must never trigger a request.

```ts
export const API_KEYS_ENDPOINT = '/api/api-key'

export const apiKeyEndpoint = (apiKeyId: string) =>
    `${API_KEYS_ENDPOINT}/${apiKeyId}`

export type ApiKey = {
    id: string
    name: string
    permissions: string[]
    createdAt: string
}

export type CreateApiKeyBody = Pick<ApiKey, 'name' | 'permissions'>
```

Paths are **absolute from the origin** and include the `/api` prefix — `env.API_URL` is the bare
origin (`http://localhost:8080`). Always build URLs from the path helpers; hand-written literals
are how stale caches happen.

Zod validation is **opt-in**. Add a schema only when the payload crosses a trust boundary or is
genuinely dynamic; then derive the type from it with `z.infer`.

Check the real backend contract with the `api-backend` skill before inventing a shape.

## 2. Write the service

`src/api/service/<domain>.ts` — one hook per operation, named for the operation.

```ts
import useSWR from 'swr'

import { API_KEYS_ENDPOINT, apiKeyEndpoint, type ApiKey, type CreateApiKeyBody } from '@/api/apiKeys'
import { useApiAction } from '@/api/utils/useApiAction'

export function useApiKeys() {
    return useSWR<ApiKey[]>(API_KEYS_ENDPOINT)
}

export function useApiKey(apiKeyId: string | undefined) {
    return useSWR<ApiKey>(apiKeyId ? apiKeyEndpoint(apiKeyId) : null)
}

export function useCreateApiKey() {
    return useApiAction<CreateApiKeyBody, ApiKey>(API_KEYS_ENDPOINT, 'POST')
}

export function useRevokeApiKey(apiKeyId: string) {
    return useApiAction<void, void>(apiKeyEndpoint(apiKeyId), 'DELETE')
}
```

Rules:

- No `fetcher` argument on read hooks — `apiFetch` is SWR's global fetcher (`src/Context.tsx`).
  Pass one only when the hook needs different error semantics, as `useCurrentUser` does to turn a
  401 into `null` rather than an error.
- A `null` key tells SWR to skip the request. Use it for conditional fetches.
- Return SWR's result object untouched (`{ data, error, isLoading, mutate }`). Do not
  destructure-and-rewrap.
- Components import from `api/service/*`, **never** from `api/client.ts` directly.
- Filtering, sorting and formatting belong in the component or a `utils/` helper.

## 3. Mutate and invalidate

```tsx
const { trigger, isMutating } = useCreateApiKey()
const { mutate } = useSWRConfig()

async function onSubmit(values: CreateApiKeyBody) {
    await trigger(values)
    await mutate(API_KEYS_ENDPOINT)
}
```

`trigger` rejects on failure — handle it explicitly and surface the message through the
notifications store with `apiErrorMessage(error, fallback)`. Never swallow it.

Prefix invalidation:

```ts
mutate((key) => typeof key === 'string' && key.startsWith(API_KEYS_ENDPOINT))
```

## 4. Add the manual mock

`src/api/service/__mocks__/<domain>.ts` — **every** exported hook must be mocked, or importers
crash on an undefined function.

```ts
import { vi } from 'vitest'

export const useApiKeys = vi.fn().mockReturnValue({
    data: [], error: undefined, isLoading: false, mutate: vi.fn(),
})

export const useCreateApiKey = vi.fn().mockReturnValue({
    trigger: vi.fn(), isMutating: false,
})
```

## 5. Test the service against MSW

`src/api/service/<domain>.test.ts` is the one place that *should* go through the network layer.

```ts
import { renderHook, waitFor } from '@testing-library/react'
import { http, HttpResponse } from 'msw'

import { API_KEYS_ENDPOINT } from '@/api/apiKeys'
import { server } from '@/test-utils/server'
import { SwrWrapper } from '@/test-utils/wrappers'

import { useApiKeys } from './apiKeys'

it('returns the api keys of the caller', async () => {
    server.use(http.get(`*${API_KEYS_ENDPOINT}`, () => HttpResponse.json([])))

    const { result } = renderHook(() => useApiKeys(), { wrapper: SwrWrapper })

    await waitFor(() =>
        expect(
            result.current.data,
            `expected data, got error: ${result.current.error}`,
        ).toBeDefined(),
    )
})
```

`SwrWrapper` provides a fresh cache and `dedupingInterval: 0` so results never leak between cases.

Also add the MSW handler that backs dev and e2e — see the `frontend-mocks` skill.

## Authentication

Auth is a backend-for-frontend OIDC flow. The browser is redirected to
`${env.API_URL}/api/auth/{login,register}?redirect=<path>`; the backend drives the OAuth exchange
and stores tokens in httpOnly cookies. The frontend never sees a token — it only reads
`/api/auth/me`, calls `/api/auth/logout`, and lets `apiFetch` retry once through
`/api/auth/refresh` on a 401. Build the entry URLs with `authRedirectUrl()` from `@/api/auth`.
