# SDK pipeline internals

Read this only when changing the codegen configuration in `frontend/scripts/generateApiSdk.ts`, or
debugging a pipeline failure that is not "the committed SDK is stale."

## Load-bearing codegen options

The config lives in `generate()` in
[frontend/scripts/generateApiSdk.ts](../../../frontend/scripts/generateApiSdk.ts). Four options
matter; know what they buy before touching them:

- **`auth: false`** — the document advertises API-key and OIDC security schemes, but the browser
  authenticates with httpOnly cookies through the backend-for-frontend. Left on, the SDK attaches
  an `Authorization` header that must not exist.
- **`enums: false`** — keeps `ApiErrorId` a plain union, which the narrowing helpers in
  `src/api/errors.ts` need.
- **`postProcess: ['prettier']`** — keeps the committed diff in house style and stable across tool
  versions. This is why the folder is in ESLint's `ignores` and **not** in `.prettierignore`.
- **`clean: true`** — a deleted endpoint must not leave an orphan file in a committed folder. It
  also wipes `README.md`, which is why the script writes that file itself after generating.

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
  otherwise generate an SDK nobody else reproduces. The scrub must still forward
  `CARGO_TARGET_DIR`, or the build lands somewhere unexpected.
- **`--check` generates inside the package**, into a gitignored `frontend/.api-sdk-check-*`.
  Prettier resolves `.prettierrc` by walking up from the files it formats, so a system temp
  directory comes back with Prettier's defaults and every file reads as changed; and Prettier
  refuses to format anything under `node_modules`.
- **The generated fetch client catches everything it throws.** A network failure does not reject;
  it returns a result whose `response` is `undefined`. `apiCall` in `src/api/client.ts` keys on
  that to raise a `NETWORK` error.
- **Schema names and operation-response names collide.** `GetApiKeyResponse` is the schema;
  `GetApiKeyResponse2` is the operation's response union. Converters want the unsuffixed one.

## What the pieces are

| Path | Role |
| --- | --- |
| `scripts/build_frontend_api_sdk.sh` | Runs both halves. `--check` verifies instead of writing |
| `scripts/test_openapi.sh` | The gate; wraps `--check`. Picked up by the pre-push hook |
| `frontend/scripts/generateApiSdk.ts` | `api:sdk` / `api:check`: the codegen config and the diff |
| `frontend/scripts/diffDirectories.ts` | Tree comparison behind `--check` |
| `frontend/openapi.json` | Build artifact. Gitignored |
| `frontend/src/api/generated/` | Output. Committed, never edited, ESLint-ignored |

`bun run api:sdk [spec]` and `bun run api:check [spec]` run the node half alone, defaulting to
`frontend/openapi.json`. They are only useful when that document already exists — a fresh clone has
none, which is why the gate is `test_openapi.sh` and not a line in `test_lint.sh`.
