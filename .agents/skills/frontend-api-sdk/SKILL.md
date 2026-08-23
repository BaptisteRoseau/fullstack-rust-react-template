---
name: frontend-api-sdk
description: How to regenerate the frontend's typed API SDK from the Rust router's OpenAPI document, and how the two-half pipeline, its committed artifact and its drift check fit together. Use this after changing crates/api, when src/api/generated is stale or missing, when scripts/test_openapi.sh fails, or when changing the codegen configuration.
---

# Frontend API SDK generation

`frontend/src/api/generated/` is not written by hand. It is produced by `@hey-api/openapi-ts` from an
OpenAPI document the Rust router emits, so the frontend's idea of the contract cannot drift from the
backend without a check failing.

```
crates/api (utoipa annotations)
   │  cargo run -p openapi_generator
   ▼
frontend/openapi.json          build artifact, gitignored, regenerated every run
   │  bun run api:sdk
   ▼
frontend/src/api/generated/    committed; the only record of the contract in the repo
```

## Regenerate after any change under crates/api

```bash
./scripts/build_frontend_api_sdk.sh
```

That is the whole procedure. It runs both halves and leaves `src/api/generated/` up to date. Commit
that folder; do **not** commit `frontend/openapi.json`.

Then, from `frontend/`, check the change landed:

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

## Adding an endpoint

1. Add it to `crates/api` with the `backend-add-api-endpoint` skill. Give every response a
   `body = ...`, including error responses: an omitted body generates as `unknown` and silently
   weakens the frontend's error typing.
2. Run `./scripts/build_frontend_api_sdk.sh`.
3. Read `src/api/generated/types.gen.ts` for the wire shape, then write the domain layer with the
   `frontend-api` skill.

If an operation you need is not in `types.gen.ts`, the backend does not have it. Add it there. Never
hand-write the path in the frontend and let an MSW handler hide the gap — that is exactly the drift
this pipeline exists to prevent.

## What the pieces are

| Path | Role |
|---|---|
| `scripts/build_frontend_api_sdk.sh` | Runs both halves. `--check` verifies instead of writing |
| `scripts/test_openapi.sh` | The gate; wraps `--check`. Picked up by the pre-push hook |
| `frontend/scripts/generateApiSdk.ts` | `api:sdk` / `api:check`: the codegen config and the diff |
| `frontend/scripts/diffDirectories.ts` | Tree comparison behind `--check` |
| `frontend/openapi.json` | Build artifact. Gitignored |
| `frontend/src/api/generated/` | Output. Committed, never edited, ESLint-ignored |

`bun run api:sdk [spec]` and `bun run api:check [spec]` run the node half alone, defaulting to
`frontend/openapi.json`. They are only useful when that document already exists — a fresh clone has
none, which is why the gate is `test_openapi.sh` and not a line in `test_lint.sh`.

## Changing the codegen configuration

The config lives in `generate()` in `frontend/scripts/generateApiSdk.ts`. Four options are
load-bearing; know what they buy before touching them:

- **`auth: false`** — the document advertises API-key and OIDC security schemes, but the browser
  authenticates with httpOnly cookies through the backend-for-frontend. Left on, the SDK attaches an
  `Authorization` header that must not exist.
- **`enums: false`** — keeps `ApiErrorId` a plain union, which the narrowing helpers in
  `src/api/errors.ts` need.
- **`postProcess: ['prettier']`** — keeps the committed diff in house style and stable across tool
  versions. This is why the folder is in ESLint's `ignores` and **not** in `.prettierignore`.
- **`clean: true`** — a deleted endpoint must not leave an orphan file in a committed folder. It also
  wipes `README.md`, which is why the script writes that file itself after generating.

`@hey-api/openapi-ts` moves its API between versions. After a version bump, verify the option names
against `node_modules/@hey-api/openapi-ts/dist/index.d.mts` rather than assuming — `output.format`
and `asClass` were both removed in past releases.

## Traps this pipeline already works around

Do not "fix" these by undoing them.

- **The generator cannot take an output path.** `Config::parse()` runs clap over the whole of
  `std::env::args()` and `CliConfig` has a positional argument, so a path passed to
  `build_openapi.sh` is swallowed by the config parser. The generator always writes
  `./openapi.json`; the shell script moves it. Do not modify `tools/openapi_generator` or
  `crates/config` to work around this.
- **The cargo half runs under `env -i`.** Config values reach the document — `servers`, and
  `openIdConnectUrl` under `securitySchemes` — so a developer with those variables exported would
  otherwise generate an SDK nobody else reproduces. The scrub must still forward `CARGO_TARGET_DIR`,
  or the build lands somewhere unexpected.
- **`--check` generates inside the package**, into a gitignored `frontend/.api-sdk-check-*`. Prettier
  resolves `.prettierrc` by walking up from the files it formats, so a system temp directory comes
  back with Prettier's defaults and every file reads as changed; and Prettier refuses to format
  anything under `node_modules`.
- **The generated fetch client catches everything it throws.** A network failure does not reject; it
  returns a result whose `response` is `undefined`. `apiCall` in `src/api/client.ts` keys on that to
  raise a `NETWORK` error.
- **Schema names and operation-response names collide.** `GetApiKeyResponse` is the schema;
  `GetApiKeyResponse2` is the operation's response union. Converters want the unsuffixed one.

## Never

- Edit anything under `src/api/generated/`.
- Commit `frontend/openapi.json`, or remove it from `.gitignore`.
- Add `bun run api:check` to `scripts/test_lint.sh`: a fresh clone has no document for it to read.
- Import `@/api/generated` outside `src/api/**` (or `src/test-utils/**`, whose MSW handlers must type
  their responses with the wire shapes).
- Tighten `operation_id` in the utoipa annotations to prettify generated names. Nothing outside
  `src/api/domains/<domain>/` may import them, so `me()` and `ping()` are harmless.
