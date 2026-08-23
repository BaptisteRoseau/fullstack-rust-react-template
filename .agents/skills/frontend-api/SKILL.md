---
name: frontend-api
description: Use when adding or changing a call to the backend — a new domain, fetcher, converter, cache key or SWR hook.
---

# Frontend API layer

`src/api/` is a four-layer client. Work downwards, never sideways — see
Skill(frontend-architecture) for the full layering rules.

```txt
api/hooks/useApiXxx/     SWR bindings. The only api entry point components use
api/domains/<domain>/    domain types, converters, fetchers, cache keys
api/client.ts errors.ts  transport, apiCall(), ApiError
api/generated/           SDK built from the backend's OpenAPI document. Never edited
```

**Never let a generated type escape `src/api/`.** ESLint blocks it: a generated type says what the
backend sends today, a domain type says what the interface needs. `converters.ts` is the only place
those meet.

## 1. Make sure the operation exists

Every fetcher calls a function from `@/api/generated`. If the endpoint is new, generate the SDK
first — Skill(frontend-api-sdk). Then read `src/api/generated/types.gen.ts` for the exact wire
shape. If the operation is still missing, the backend does not have it: add it with the
`backend-add-api-endpoint` skill. Never invent it in the frontend and let MSW hide the gap.

## 2. Pick the domain

A domain is **one noun the interface reasons about**, not a backend tag or a URL prefix. It owns
that noun's type, converters, keys, and every fetcher whose input or output is that noun — whatever
path it comes from.

Scaffold it with `bun run generate api <domainName> <backendPath>` — it writes to
`src/api/domains/<domain>/`. See
[src/api/domains/apiKeys](../../../frontend/src/api/domains/apiKeys) for the reference shape:
`apiKeys.ts` (fetchers), `types.ts`, `converters.ts`, `keys.ts`, both `.test.ts` files, `index.ts`.
The barrel exports fetchers, keys and types — **never converters**.

## 3. Write the domain type first

Hand-write it in `types.ts`. Never alias or re-export a generated type. See
[src/api/domains/apiKeys/types.ts](../../../frontend/src/api/domains/apiKeys/types.ts).

## 4. Write the converter

This is where the real decisions live:

- **Narrow weak wire types.** `string[]` becomes the union the UI needs.
- **Filter unknown values, never reject them.** A permission the backend adds tomorrow must not
  blank the table.
- **Normalise units at the boundary.** One endpoint sends RFC 3339, another an epoch in
  milliseconds; both become `Date`. The interface must not know which.
- **Rename what is dangerous.** The wire's `key` becomes `secret`; `key` next to SWR cache keys is
  a landmine.

See [src/api/domains/apiKeys/converters.ts](../../../frontend/src/api/domains/apiKeys/converters.ts)
for `fromGetApiKeyResponse`, which applies all four. Test converters with no MSW — see
[converters.test.ts](../../../frontend/src/api/domains/apiKeys/converters.test.ts) — and assert the
unknown-value case.

## 5. Write the fetchers

Verb-first and `Promise`-returning. Reads are `fetch*`; mutations take the domain verb. Everything
goes through `apiCall`, and the generated function is imported under an alias so the layer boundary
stays visible:

```ts
import { createApiKey as createApiKeyRequest, getApiKey } from '@/api/generated'
```

See [src/api/domains/apiKeys/apiKeys.ts](../../../frontend/src/api/domains/apiKeys/apiKeys.ts).

**Endpoint-specific semantics belong in the fetcher, not in a hook or a component.** A 401 from
`GET /auth/me` means "signed out", so
[`fetchCurrentUser`](../../../frontend/src/api/domains/currentUser/currentUser.ts) answers `null`
instead of throwing.

## 6. Write the cache keys

Tuples, `as const`, one factory per domain, never a URL string. See
[src/api/domains/apiKeys/keys.ts](../../../frontend/src/api/domains/apiKeys/keys.ts).

## 7. Write the hook

Run `bun run generate api-hook <OperationName> <domainName>`. Name it `useApi` + operation,
**always**, including the stutter: `useApiApiKey`. The prefix is the point — it hides a network
request behind what otherwise reads like `useBooleanState()`.

A read hook takes its argument **out of the key**, so key and request cannot disagree; a `null` key
skips the request — see
[useApiApiKey.ts](../../../frontend/src/api/hooks/useApiApiKey/useApiApiKey.ts).

**A mutation hook owns its invalidation.** No component should have to remember to refresh a list.
Give each mutation its own mutation key — two hooks sharing one key share their `isMutating` state.
See [useApiCreateApiKey.ts](../../../frontend/src/api/hooks/useApiCreateApiKey/useApiCreateApiKey.ts).
Return SWR's result object untouched — never destructure and rewrap it.

## 8. Handle errors by cause

Every failure arrives as an `ApiError` carrying an `id`, whether it came from the backend
(`NOT_FOUND`, `TOKEN_EXPIRED`, …), from a dead connection (`NETWORK`) or from a body outside the
contract (`PARSE`). Branch on the id with `matchApiError`, never on the status code — a 401 is both
"not signed in" and "session expired". Show the user `useApiErrorMessage()`, which is translated;
the backend's `error` string is English and belongs in logs only. See
[src/api/errors.ts](../../../frontend/src/api/errors.ts).

## 9. Mock, then verify

Add the MSW handler and register it in `src/test-utils/mocks/handlers/index.ts` —
Skill(frontend-mocks). Then, from `frontend/`, run the checklist below.

## Never

- Hand-write a path, or call `fetch` outside `api/client.ts`.
- Import `@/api/generated` outside `src/api/**` (or `src/test-utils/**`).
- Import past a domain barrel: `@/api/domains/apiKeys/converters` is out of bounds.
- Export converters from a domain's `index.ts`.
- Alias or re-export a generated type as a domain type.
- Call `mutate` by hand in a component for something a mutation hook already owns.
- Put a formatter or a derivation in the api layer — that belongs in `src/utils/`.

## Checklist

```bash
./scripts/build_frontend_api_sdk.sh --check
bun run i18n:check     # if you added user-facing strings (error messages, labels)
```

- [ ] The domain's `index.ts` exports fetchers, keys and types — never converters.
- [ ] Every MSW response for this domain is typed with the generated (wire) type, not the domain type.
