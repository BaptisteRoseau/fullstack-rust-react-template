# api

The HTTP client, in four layers: `generated/` (the SDK built from the backend's OpenAPI document),
`client.ts` + `errors.ts` (transport and error contract), `domains/<domain>/` (domain types,
converters and fetchers), and `hooks/` (the SWR bindings).

Nothing above `src/api/` may name a wire type: import the domain type from
`@/api/domains/<domain>` or call `@/api/hooks/useApiXxx`. ESLint enforces it.

## Regenerating the SDK

After any change under `crates/api`:

```bash
./scripts/build_frontend_api_sdk.sh    # regenerate openapi.json and generated/
./scripts/test_openapi.sh              # verify the committed SDK matches the router
```

## Adding to the layer

```bash
bun run generate      # "api" scaffolds a domain, "api-hook" scaffolds an SWR binding
```

See [01 – API layer](../../docs/architecture/01-api.md) for the rules, and the `frontend-api` and
`frontend-api-sdk` skills for the procedures.
