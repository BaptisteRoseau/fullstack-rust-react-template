# `config/`

Global configuration.

| File | Role |
|------|------|
| `env.ts` | Validates and types env vars with Zod. Reads `VITE_APP_*` from `import.meta.env`, strips the prefix, and throws at startup if anything is missing/invalid. Import the typed `env` object (`env.API_URL`, `env.ENABLE_API_MOCKING`, …). |
| `paths.ts` | **The single source of truth for routes.** Every route exposes a `path` (for the router) and a `getHref(...)` builder (for links/redirects). |

## Rules

- **Never hardcode a URL** — use `paths.<area>.<name>.getHref(...)`. Add new routes here first.
- **Never read `import.meta.env` directly** in app code — add the variable to the `EnvSchema` in `env.ts` and consume the typed `env`. New vars must be prefixed `VITE_APP_` to be picked up.

See `.claude/skills/frontend-react-page` for how `paths.ts` feeds the router.
