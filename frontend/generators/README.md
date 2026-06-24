# `generators/`

**Plop** code generators (wired in `plopfile.cjs`). Run with `bun run generate` (then pick a
generator) or `bun run generate <name>` to run one directly.

```
generators/<name>/
├── index.cjs        # generator definition + prompts
└── *.hbs            # Handlebars templates (one per emitted file)
```

Each generator scaffolds the minimal, runnable shape for one kind of object and mirrors an existing
example in the codebase. Output is **intentionally minimal** — flesh it out following the matching
`.claude/skills/frontend-react-*` skill. To change a scaffold, edit its `.hbs` templates; to add a
generator, drop a folder here and register it in `plopfile.cjs`.

## Generators

| Name        | Emits                                                                                                           | Skill                      |
| ----------- | --------------------------------------------------------------------------------------------------------------- | -------------------------- |
| `component` | `components/<name>/{<name>.tsx, .stories.tsx, index.ts}` under `src/components/<folder>` or a feature           | `frontend-react-component` |
| `page`      | A `ContentLayout` page under `src/app/pages/app/`, **plus** a `paths.ts` entry and a lazy route in `router.tsx` | `frontend-react-page`      |
| `layout`    | `src/components/layouts/<name>-layout.tsx`, re-exported from the barrel                                         | `frontend-react-layout`    |
| `feature`   | A new `src/features/<name>/` slice (read endpoint + list component)                                             | `frontend-react-feature`   |
| `hook`      | `use-<name>.ts` in `src/hooks` (+ colocated test) or a feature's `hooks/`                                       | `frontend-react-hook`      |
| `api`       | One endpoint file in a feature's `api/` — `query` (read) or `mutation` (write)                                  | `frontend-react-api`       |
| `form`      | A `Create<Noun>` FormDrawer component in a feature's `components/`                                              | `frontend-react-form`      |
| `store`     | A Zustand `<name>-store.ts` in a feature or component folder                                                    | `frontend-react-state`     |

### Notes

- **`component` / `store`** prompt for a target: `components` (then a subfolder under
  `src/components/`) or one of the existing features.
- **`page`** is the only generator that edits existing files — it appends to `src/config/paths.ts`
  and `src/app/router.tsx` (authenticated `/app` route). Add the sidebar nav link and any
  `clientLoader` by hand per the skill.
- **`feature`** scaffolds only the common folders. Remember to register the import boundary in
  `eslint.config.cjs` and add the domain model to `@/types/api.ts`.
- **`api` / `form`** pair up: generate the `create` mutation first, then the form reuses its
  `create<Noun>InputSchema` + `useCreate<Noun>`. Swap the placeholder `unknown` payload types for the
  real domain model, and add a matching MSW handler (`frontend-react-mocks`) so calls work in
  dev/tests.
