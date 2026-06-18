# `types/`

Shared TypeScript types.

- `api.ts` — the **domain models** returned by the API (`User`, `Team`, `Discussion`, `Comment`,
  `AuthResponse`, `Meta`, plus `BaseEntity`/`Entity` helpers). Conceptually generated from / kept in
  sync with the Rust backend.

## Rules

- **API response shapes shared across features go here**, not inside a feature. Feature `api/` files and
  components import these (`import { Discussion, Meta } from '@/types/api'`).
- Component- or feature-local view types stay with their component (`types.ts`) or in the feature's `types/`.
- Treat `api.ts` as the contract with the backend — when the backend model changes, update it here so
  every consumer is type-checked.
