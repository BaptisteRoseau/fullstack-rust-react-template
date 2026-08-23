# generated

**Do not edit anything in this folder.** It is the output of `@hey-api/openapi-ts`, run against an
OpenAPI document produced by the Rust router, and it is overwritten wholesale on every run.

Its files are left read-only for that reason; regenerate rather than chmod.

```bash
./scripts/build_frontend_api_sdk.sh    # regenerate
./scripts/test_openapi.sh              # fails if this folder no longer matches the router
```

The document it is built from, `frontend/openapi.json`, is a build artifact and is not committed.
This folder is: it is what `tsc`, Vite and Vitest import, and a frontend-only clone has no cargo to
rebuild it.

Import it only from `src/api/**` -- `converters.ts` for the wire types, `<domain>.ts` for the
operations. ESLint blocks it everywhere else, except `src/test-utils/**`, whose MSW handlers must
type their responses with the wire shapes.
