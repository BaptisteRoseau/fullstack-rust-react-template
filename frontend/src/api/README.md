# api

The HTTP client, in four layers, each only allowed to see the one below it:

```txt
api/
├── generated/          # SDK built from the backend's OpenAPI document — never edited by hand
├── client.ts           # transport: apiCall(), fetchWithSessionRefresh
├── errors.ts           # ApiError, matchApiError, useApiErrorMessage
├── domains/
│   └── <domain>/        # one noun the interface reasons about (apiKeys, currentUser, users, session)
│       ├── <domain>.ts            # fetchers — the only file in the domain calling generated
│       ├── types.ts               # hand-written domain types
│       ├── converters.ts          # fromApi*/toApi* — the only file naming generated *types*
│       ├── keys.ts                # SWR cache-key factory
│       └── index.ts                # barrel: fetchers, keys, types — never converters
└── hooks/
    └── useApiXxx/           # one SWR binding per operation, named useApi<Operation>
```

Nothing above `src/api/` may name a wire type from `generated/`: import the domain type from
`@/api/domains/<domain>` or call `@/api/hooks/useApiXxx` instead. ESLint enforces it, and
`src/test-utils/**` is the one exception (its MSW handlers must type responses with the wire
shape).

A domain's `index.ts` never exports `converters.ts` — nothing outside the domain folder may see a
converter directly.

## Skills

- [frontend-api](../../../.claude/skills/frontend-api/SKILL.md)
- [frontend-api-sdk](../../../.claude/skills/frontend-api-sdk/SKILL.md)
