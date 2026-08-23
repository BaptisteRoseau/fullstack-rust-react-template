---
name: frontend-api
description: How to add or change a call to the backend in src/api — a domain folder (types, converters, fetchers, cache keys) and its SWR binding under api/hooks. Use this when adding or updating a frontend API call, query, mutation, cache invalidation or error handling.
---

# Frontend API layer

`src/api/` is a four-layer client. Work downwards, never sideways:

```
api/hooks/useApiXxx/     SWR bindings. The only api entry point components use
api/domains/<domain>/    domain types, converters, fetchers, cache keys
api/client.ts errors.ts  transport, apiCall(), ApiError
api/generated/           SDK built from the backend's OpenAPI document. Never edited
```

**Never let a generated type escape `src/api/`.** ESLint blocks it, but understand why: a generated
type says what the backend sends today, a domain type says what the interface needs. `converters.ts`
is the only place those meet.

## 1. Make sure the operation exists

Every fetcher calls a function from `@/api/generated`. If the endpoint is new, generate it first —
**do not hand-write a path**:

```bash
./scripts/build_frontend_api_sdk.sh
```

Then read `src/api/generated/types.gen.ts` for the exact wire shape. If the operation is missing, the
backend does not have it: add it with the `backend-add-api-endpoint` skill. Never invent it in the
frontend and let MSW hide the gap. See the `frontend-api-sdk` skill.

## 2. Pick the domain

A domain is **one noun the interface reasons about**, not a backend tag or a URL prefix. It owns that
noun's type, converters, keys, and every fetcher whose input or output is that noun — whatever path
it comes from. If your folders mirror the OpenAPI tags, the converter layer is decorative.

Scaffold one with `bun run generate` → `api`; it writes to `src/api/domains/<domain>/`. These are
the **only** filenames allowed there:

| File | Required when | Contents |
|---|---|---|
| `<domain>.ts` | always | the fetchers; the only file calling `generated` |
| `<domain>.test.ts` | always | MSW-backed; fetcher + converter together |
| `types.ts` | the domain has a payload | hand-written types; may export an `as const` union source |
| `converters.ts` | `types.ts` exists | `fromApi*` / `toApi*`; the only file importing generated **types** |
| `converters.test.ts` | `converters.ts` exists | pure, no MSW |
| `keys.ts` | something is read through SWR | the cache-key factory |
| `index.ts` | always | barrel: fetchers, keys, types. **Never** converters |

## 3. Write the domain type first

Hand-write it. Never alias a generated type, never re-export one.

```ts
// src/api/domains/apiKeys/types.ts
export const API_KEY_PERMISSIONS = ['read', 'write', 'admin'] as const
export type ApiKeyPermission = (typeof API_KEY_PERMISSIONS)[number]

export type ApiKey = {
    id: string
    name: string
    permissions: ApiKeyPermission[]
    createdAt: Date
}
```

## 4. Write the converter

This is where the real decisions live. Make them deliberately:

```ts
// src/api/domains/apiKeys/converters.ts
import type { GetApiKeyResponse } from '@/api/generated'

export function fromGetApiKeyResponse(response: GetApiKeyResponse): ApiKey {
    return {
        id: response.id,
        name: response.name,
        permissions: response.permissions.filter(isApiKeyPermission),
        createdAt: new Date(response.createdAt),
    }
}
```

- **Narrow weak wire types.** `string[]` becomes the union the UI needs.
- **Filter unknown values, never reject them.** A permission the backend adds tomorrow must not blank
  the table.
- **Normalise units at the boundary.** One endpoint sends RFC 3339, another an epoch in
  milliseconds; both become `Date`. The interface must not know which.
- **Rename what is dangerous.** The wire's `key` becomes `secret`; `key` next to SWR cache keys is a
  landmine.

Test converters with no MSW. Assert the decisions, including the unknown-value case.

## 5. Write the fetchers

Verb-first and `Promise`-returning. Reads are `fetch*`; mutations take the domain verb. Everything
goes through `apiCall`.

```ts
// src/api/domains/apiKeys/apiKeys.ts
import { apiCall } from '@/api/client'
import { createApiKey as createApiKeyRequest, getApiKey } from '@/api/generated'

export async function fetchApiKey(apiKeyId: string): Promise<ApiKey> {
    return fromGetApiKeyResponse(
        await apiCall(() => getApiKey({ path: { id: apiKeyId } })),
    )
}

export async function createApiKey(apiKey: NewApiKey): Promise<CreatedApiKey> {
    return fromCreateApiKeyResponse(
        await apiCall(() =>
            createApiKeyRequest({ body: toCreateApiKeyRequest(apiKey) }),
        ),
    )
}
```

The import alias on the generated `createApiKey` is the layer proving it exists: theirs speaks wire
types, ours speaks domain types.

**Endpoint-specific semantics belong in the fetcher, not in a hook or a component.** A 401 from
`GET /auth/me` means "signed out", so `fetchCurrentUser` answers `null` instead of throwing.

## 6. Write the cache keys

```ts
export const apiKeyKeys = {
    all: ['apiKeys'] as const,
    detail: (apiKeyId: string) => ['apiKeys', apiKeyId] as const,
}
```

Tuples, `as const`, one factory per domain. Never a URL string.

## 7. Write the hook

`bun run generate` → `api-hook`. One folder per hook under `src/api/hooks/`, mirroring `src/hooks/`:
`useApiXxx.ts`, `useApiXxx.test.ts`, `index.ts`.

Name it `useApi` + operation, **always**, including the stutter: `useApiApiKey`. The prefix is the
point — `const { data } = useApiKeys()` looks exactly like `useBooleanState()` and hides a network
request.

A read hook takes its argument **out of the key**, so key and request cannot disagree. A `null` key
skips the request:

```ts
export function useApiApiKey(apiKeyId: string | undefined) {
    return useSWR(apiKeyId ? apiKeyKeys.detail(apiKeyId) : null, ([, id]) =>
        fetchApiKey(id),
    )
}
```

**A mutation hook owns its invalidation.** No component should have to remember to refresh a list.

```ts
const MUTATION_KEY = ['apiKeys', 'create'] as const

export function useApiCreateApiKey() {
    const { mutate } = useSWRConfig()

    return useSWRMutation(
        MUTATION_KEY,
        (_key, { arg }: { arg: NewApiKey }) => createApiKey(arg),
        { onSuccess: () => void mutate(apiKeyKeys.all) },
    )
}
```

Give each mutation a key of its own: two mutation hooks sharing one key share their `isMutating`
state. Return SWR's result object untouched — never destructure and rewrap.

## 8. Handle errors by cause

Every failure arrives as an `ApiError` carrying an `id`, whether it came from the backend
(`NOT_FOUND`, `TOKEN_EXPIRED`, …), from a dead connection (`NETWORK`) or from a body outside the
contract (`PARSE`).

Branch on the id, never on the status code — a 401 is both "not signed in" and "session expired":

```ts
matchApiError(error, {
    TOKEN_EXPIRED: () => refresh(),
    default: () => signOut(),
})
```

Show the user `useApiErrorMessage()`, which is translated. The backend's `error` string is English
and belongs in logs:

```tsx
const apiErrorMessage = useApiErrorMessage()

addNotification({
    type: 'error',
    title: t`Could not revoke the API key`,
    message: apiErrorMessage(error),
})
```

## 9. Mock, then verify

Add the MSW handler (`frontend-mocks` skill) and register it in
`src/test-utils/mocks/handlers/index.ts`. **Handlers emit wire shapes, not domain shapes** — type
them with the generated types so the compiler holds the line:

```ts
return HttpResponse.json<GetApiKeyResponse>(toGetApiKeyResponse(apiKey))
```

Then, from `frontend/`:

```bash
bun run check-types
bun run lint
bun run test
bun run i18n:check     # if you added user-facing strings
```

## Never

- Hand-write a path, or call `fetch` outside `api/client.ts`.
- Import `@/api/generated` outside `src/api/**` (or `src/test-utils/**`).
- Import past a domain barrel: `@/api/domains/apiKeys/converters` is out of bounds.
- Export converters from a domain's `index.ts`.
- Alias or re-export a generated type as a domain type.
- Call `mutate` by hand in a component for something a mutation hook already owns.
- Put a formatter or a derivation in the api layer — that is `src/utils/`.
