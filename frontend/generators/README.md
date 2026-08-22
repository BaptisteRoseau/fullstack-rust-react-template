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
| `api` | `api/<domain>.ts`, its service, manual mock, MSW-backed test and mock handler |
| `hook` | A `src/hooks/<useName>/` folder with the hook, its test and a barrel |
| `store` | A Zustand store in `src/stores/` |

A generator only writes files. You still have to wire the result up:

- a page needs its `PATHS` entry, its lazy route in `src/router/routes.tsx` and a nav link;
- an api domain needs its handler registered in `src/test-utils/mocks/handlers/index.ts`;
- any user-facing string needs a Lingui macro, then `bun run i18n:extract`.

See `frontend/docs/architecture/` for the rules the templates follow.
