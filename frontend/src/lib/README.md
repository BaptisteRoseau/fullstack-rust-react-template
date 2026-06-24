# `lib/`

Single, app-wide **preconfigured instances** of third-party libraries. Configure once here, import
everywhere — never reconfigure these in components.

| File                | Provides                                                                                                                                                                                                   |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api-client.ts`     | The one Axios instance `api`. Request interceptor sets `Accept` + `withCredentials`; response interceptor **unwraps `.data`**, toasts errors via the notifications store, and redirects to login on `401`. |
| `react-query.ts`    | `queryConfig` defaults + the `QueryConfig` / `MutationConfig` generic helpers used by every feature `api/` hook.                                                                                           |
| `auth.tsx`          | Auth fetchers + Zod schemas + `configureAuth` → `useUser`/`useLogin`/`useLogout`/`useRegister`/`AuthLoader`, and `ProtectedRoute`.                                                                         |
| `authorization.tsx` | `ROLES` (RBAC), `POLICIES` (PBAC), `useAuthorization`, and the `<Authorization>` gate.                                                                                                                     |

## Rules

- **All HTTP goes through `api`** from `api-client.ts` — never import raw `axios`/`fetch` in features. Because the interceptor unwraps `.data`, fetcher return types reflect the response envelope (often `{ data, meta }`).
- Auth lives here (not in `features/auth`) because it's shared across features; `features/auth` only has the form components.
- Add roles to the `ROLES` enum and permissions to `POLICIES` — don't inline `user.role === ...` checks.

See `.claude/skills/frontend-react-api` and `frontend-react-authorization`.
