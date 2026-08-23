---
name: frontend-api-sdk
description: Use after changing crates/api, when src/api/generated is stale or missing, or when scripts/test_openapi.sh fails.
---

# Frontend API SDK generation

`frontend/src/api/generated/` is not written by hand. It is produced by `@hey-api/openapi-ts` from
an OpenAPI document the Rust router emits, so the frontend's idea of the contract cannot drift from
the backend without a check failing.

```txt
crates/api (utoipa annotations)
   │  cargo run -p openapi_generator
   ▼
frontend/openapi.json          build artifact, gitignored, regenerated every run
   │  bun run api:sdk
   ▼
frontend/src/api/generated/    committed; the only record of the contract in the repo
```

## 1. Add the endpoint to the backend

Use the `backend-add-api-endpoint` skill. Give every response a `body = ...`, including error
responses — an omitted body generates as `unknown` and silently weakens the frontend's error typing.

## 2. Regenerate the SDK

```bash
./scripts/build_frontend_api_sdk.sh
```

That is the whole procedure. It runs both halves and leaves `src/api/generated/` up to date. Commit
that folder; do **not** commit `frontend/openapi.json`.

## 3. Read the wire shape, then write the domain layer

Read `src/api/generated/types.gen.ts` for the exact shape, then write the fetchers, types and
converters with Skill(frontend-api). If the operation you need is still missing from
`types.gen.ts`, the backend does not have it — go back to step 1. Never hand-write the path in the
frontend and let an MSW handler hide the gap.

## 4. Verify

```bash
bun run check-types
bun run lint
bun run test
```

## Verify without writing

```bash
./scripts/test_openapi.sh
```

Regenerates the document, generates the SDK into a scratch directory, and diffs it against the
committed folder. It is the single enforced invariant: **the committed SDK matches an OpenAPI
document regenerated from the current router.** The `pre-push` hook runs it via its
`scripts/test_*.sh` glob.

When it fails, the fix is always `./scripts/build_frontend_api_sdk.sh` followed by a commit of
`src/api/generated/` — never an edit inside that folder.

Changing the codegen configuration, or debugging the pipeline itself, is rare — read
[internals.md](./internals.md) only when you need to do that.

## Never

- Edit anything under `src/api/generated/`.
- Commit `frontend/openapi.json`, or remove it from `.gitignore`.
- Add `bun run api:check` to `scripts/test_lint.sh`: a fresh clone has no document for it to read.
- Import `@/api/generated` outside `src/api/**` (or `src/test-utils/**`, whose MSW handlers must
  type their responses with the wire shapes).
- Tighten `operation_id` in the utoipa annotations to prettify generated names. Nothing outside
  `src/api/domains/<domain>/` may import them, so `me()` and `ping()` are harmless.

## Checklist

```bash
./scripts/test_openapi.sh
```

- [ ] `src/api/generated/` is committed in the same commit as the `crates/api` change.
- [ ] `frontend/openapi.json` is not staged.
