# Generators

Plop templates that scaffold a folder in the shape the architecture expects. Run them from
`frontend/`:

```bash
bun run generate
```

| Generator | Creates |
|---|---|
| `component` | A `design-system/` primitive (with story) or a shared `components/` component |
| `page` | A `src/pages/<Name>/` folder with page, stylesheet, test and barrel |
| `api` | An `api/domains/<domain>/` folder: fetchers, domain types, converters, cache keys, both tests, and its MSW handler |
| `api-hook` | An `api/hooks/useApiXxx/` folder with the SWR binding, its test and a barrel |
| `hook` | A `src/hooks/<useName>/` folder with the hook, its test and a barrel |
| `store` | A Zustand store in `src/stores/` |

A generator only writes files. You still have to wire the result up:

- a page needs its `PATHS` entry, its lazy route in `src/router/routes.tsx` and a nav link;
- an api domain needs its handler registered in `src/test-utils/mocks/handlers/index.ts`, and its
  fetchers must call operations that exist in `src/api/generated` — run
  `./scripts/build_frontend_api_sdk.sh` first if the endpoint is new;
- any user-facing string needs a Lingui macro, then `bun run i18n:extract`.

See `frontend/docs/architecture/` for the rules the templates follow.
