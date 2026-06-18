# `utils/`

Shared **pure** helper functions. No React, no side effects, no API/feature imports.

- `cn.ts` — `cn(...)`, the `clsx` + `tailwind-merge` class combiner used by every UI component to merge variant classes with a caller's `className`.
- `format.ts` — formatting helpers (e.g. `formatDate`, backed by `dayjs`).

Keep functions here generic and unit-testable; colocate a `*.test.ts` for non-trivial logic.
