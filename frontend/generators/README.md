# generators

Plop templates that scaffold a new folder in the exact shape the architecture expects. Registered
in [`plopfile.cjs`](../plopfile.cjs) and run with `bun run generate`.

```txt
generators/
└── <generator-name>/       # component, page, api, api-hook, hook, store
    ├── index.cjs           # prompts and the list of files it writes
    └── *.hbs                # one Handlebars template per generated file
```

| Generator | Writes |
| --- | --- |
| `component` | A `design-system/` primitive (with a story) or a shared `components/` component |
| `page` | A `src/pages/<Name>/` folder: page, stylesheet, test and barrel |
| `api` | An `api/domains/<domain>/` folder: fetchers, domain types, converters, cache keys, both tests, and its MSW handler |
| `api-hook` | An `api/hooks/useApiXxx/` folder: the SWR binding, its test and a barrel |
| `hook` | A `src/hooks/<useName>/` folder: the hook, its test and a barrel |
| `store` | A Zustand store file in `src/stores/` |

A generator only writes files; it never edits an existing one, and it does not register the result
anywhere else in the app.

## Skills

- [frontend-architecture](../../.claude/skills/frontend-architecture/SKILL.md)
