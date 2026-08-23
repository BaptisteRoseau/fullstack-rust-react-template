# 01 – API layer

← [Back to overview](README.md)

`src/api/` owns every HTTP call to the Rust backend. It is a real client, in four layers:

| Layer | What it is | Who may import it |
|---|---|---|
| `generated/` | The SDK produced from the backend's OpenAPI document. Never hand-edited | `src/api/**` only |
| `client.ts` · `errors.ts` | Transport and the error contract | `src/api/**` |
| `domains/<domain>/` | Domain types, converters and `Promise`-returning fetchers | anything, through the barrel |
| `hooks/` | The SWR bindings. Their names announce that they hit the network | anything |

The split exists so that **nothing above `src/api/` ever sees a wire type**. A generated type
describes what the backend happens to send today; a domain type describes what the interface needs.
The converter between them is the only place those two facts meet.

---

## Directory tree

```
src/api/
├── generated/                  # OUTPUT of the codegen. Committed, never edited
├── client.ts                   # configures the generated client + apiCall()
├── errors.ts                   # ApiError, narrowing helpers, translated messages
├── domains/
│   └── <domain>/
│       ├── <domain>.ts         # the fetchers — the only file calling generated/
│       ├── <domain>.test.ts    # MSW-backed: fetcher + converter together
│       ├── types.ts            # hand-written domain types
│       ├── converters.ts       # fromApi* / toApi* — the only file importing generated types
│       ├── converters.test.ts  # pure, no MSW
│       ├── keys.ts             # SWR cache-key factory
│       └── index.ts            # barrel: fetchers, keys, types. NEVER converters
└── hooks/
    └── useApiXxx/
        ├── useApiXxx.ts
        ├── useApiXxx.test.ts
        └── index.ts
```

Those are the **only** filenames allowed under `src/api/domains/<domain>/`.

| File | Required when |
|---|---|
| `<domain>.ts`, `<domain>.test.ts`, `index.ts` | always |
| `types.ts` | the domain has a payload |
| `converters.ts`, `converters.test.ts` | `types.ts` exists |
| `keys.ts` | something is read through SWR |
| `constants.ts` | optional, for constants that are not union sources |

---

## The generated SDK (`api/generated/`)

Produced by `@hey-api/openapi-ts` from an OpenAPI document the Rust router emits. Regenerate after
any change under `crates/api`:

```bash
./scripts/build_frontend_api_sdk.sh          # regenerate
./scripts/test_openapi.sh                    # verify the committed SDK still matches the router
```

`openapi.json` is a build artifact and is **not** committed; `src/api/generated/` is. See
[`src/api/README.md`](../../src/api/README.md) and the `frontend-api-sdk` skill.

---

## What a domain is

**One noun the interface reasons about, with one domain type at its centre.** Not a backend tag, not
a URL prefix, not a Rust module. The folder owns that noun's type, converters, keys and every fetcher
whose input or output is that noun — whichever backend path it comes from.

If the folders mirror the OpenAPI tags, the converter layer is decorative. The backend's single
`Authentication` tag is deliberately split in two here: `currentUser` is a resource the interface
renders, `session` is a pair of actions that return nothing.

Naming:

- **Domain folder** — camelCase, under `domains/`, named after the *frontend* noun.
- **Fetchers** — verb-first, `Promise`-returning. Reads use `fetch*`; mutations use the domain verb
  (`createApiKey`, `revokeApiKey`, `logout`).
- **Converters** — `fromApi<Thing>` / `to<Wire>Request`, named after the generated type they touch.
- **Cache keys** — a `<domain>Keys` object of `as const` tuples.
- **Hooks** — `useApi` + operation. Always, including the stutter: `useApiApiKey`.

---

## The transport (`api/client.ts`)

Configures the generated client once, at module load, and exposes one unwrapper.

```ts
client.setConfig({
    baseUrl: `${env.API_URL}/api`,
    credentials: 'include',
    fetch: fetchWithSessionRefresh,
})
```

`fetchWithSessionRefresh` renews an expired session once and replays the request. It wraps `fetch`
rather than using the generated client's interceptors, so the behaviour survives a code-generator
upgrade and can be tested on its own.

`apiCall` is the single unwrapper every fetcher goes through:

```ts
export async function fetchApiKey(apiKeyId: string): Promise<ApiKey> {
    return fromGetApiKeyResponse(
        await apiCall(() => getApiKey({ path: { id: apiKeyId } })),
    )
}
```

It returns the payload on success and throws an `ApiError` on every failure — including the ones the
backend never saw, because the generated client catches its own throws and reports a network failure
as a result with no `response`.

---

## Errors (`api/errors.ts`)

`ApiError` carries `status`, the raw `body`, and an `id`:

```ts
type AnyApiErrorId = ApiErrorId | 'NETWORK' | 'PARSE'
```

`ApiErrorId` comes from the generated schema; `NETWORK` and `PARSE` cover what the backend cannot
report. The body is parsed with Zod, so a response outside the error contract becomes `PARSE` rather
than a guess.

Branch on the cause, never on the status code — a 401 is both "not signed in" and "session expired":

```ts
matchApiError(error, {
    TOKEN_EXPIRED: () => refresh(),
    default: () => signOut(),
})
```

And render `useApiErrorMessage()`, which maps the id to a **translated** string. The backend's own
`error` field is English and belongs in logs:

```tsx
const apiErrorMessage = useApiErrorMessage()
// …
addNotification({
    type: 'error',
    title: t`Could not revoke the API key`,
    message: apiErrorMessage(error),
})
```

---

## A domain, end to end

```ts
// src/api/domains/apiKeys/types.ts — nothing here derives from a generated type
export const API_KEY_PERMISSIONS = ['read', 'write', 'admin'] as const
export type ApiKeyPermission = (typeof API_KEY_PERMISSIONS)[number]

export type ApiKey = {
    id: string
    name: string
    permissions: ApiKeyPermission[]
    createdAt: Date
}

export type CreatedApiKey = ApiKey & { secret: string }
export type NewApiKey = Pick<ApiKey, 'name' | 'permissions'>
```

```ts
// src/api/domains/apiKeys/converters.ts — the only file here importing @/api/generated
export function fromGetApiKeyResponse(response: GetApiKeyResponse): ApiKey {
    return {
        id: response.id,
        name: response.name,
        permissions: response.permissions.filter(isApiKeyPermission),
        createdAt: new Date(response.createdAt),
    }
}
```

Three shaping decisions live there, none of them generatable:

- the wire's `string[]` narrows to the union the checkbox group needs, **filtering** unknown values
  rather than rejecting them — a permission the backend adds tomorrow must not blank the table;
- `createdAt` becomes a `Date`. This resource sends RFC 3339 while `GetMeResponse` sends an epoch in
  milliseconds; the interface must not care;
- the wire's `key` becomes `secret`, because `key` next to SWR cache keys is a landmine.

```ts
// src/api/domains/apiKeys/keys.ts
export const apiKeyKeys = {
    all: ['apiKeys'] as const,
    detail: (apiKeyId: string) => ['apiKeys', apiKeyId] as const,
}
```

```ts
// src/api/domains/apiKeys/index.ts
export * from './apiKeys'
export * from './keys'
export * from './types'
// converters stay out — nothing above src/api may call them
```

---

## Hooks (`api/hooks/useApiXxx/`)

One folder per hook, mirroring `src/hooks/` exactly. Consumers import
`@/api/hooks/useApiXxx`, never the inner file.

A read hook takes its argument **out of the key**, so the key and the request cannot disagree, and a
`null` key skips the request:

```ts
export function useApiApiKey(apiKeyId: string | undefined) {
    return useSWR(apiKeyId ? apiKeyKeys.detail(apiKeyId) : null, ([, id]) =>
        fetchApiKey(id),
    )
}
```

**A mutation hook owns its invalidation.** No call site should have to remember to refresh a list;
forgetting is silent.

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

The mutation key is the hook's own, never a read key shared with another mutation: two mutation
hooks on one key share their `isMutating` state.

Return SWR's result object untouched. Do not destructure and rewrap.

---

## Enforcement

`eslint.config.cjs` makes the layering mechanical, not a convention:

- `@/api/generated` is unreachable outside `src/api/**` — except `src/test-utils/**`, whose MSW
  handlers must type their responses with the wire types;
- anything below a barrel is unreachable outside `src/api/**` — `@/api/domains/<domain>/<file>` and
  `@/api/hooks/useApiXxx/<file>` alike. The two barrels, `@/api/domains/<domain>` and
  `@/api/hooks/useApiXxx`, are the public names;
- `src/design-system/**` may not touch `@/api/*` at all.

---

## Testing

| File | Backed by | Asserts |
|---|---|---|
| `converters.test.ts` | nothing | the shaping decisions: units, unions, renames |
| `<domain>.test.ts` | MSW | fetcher and converter together, plus the error mapping |
| `useApiXxx.test.ts` | MSW + `SwrWrapper` | keying, skipping, and that mutations invalidate |

Component tests stay off the network with `vi.mock('@/api/hooks/useApiXxx')`: the automock plus
`vi.mocked(useApiApiKeys).mockReturnValue(...)` needs no hand-maintained double.

MSW handlers emit **wire** shapes. This is the easiest trap in the layer — `apiKey.createdAt` is a
string on the wire and a `Date` in the domain — so type them with the generated types and let the
compiler hold the line:

```ts
return HttpResponse.json<GetApiKeyResponse>(toGetApiKeyResponse(apiKey))
```
