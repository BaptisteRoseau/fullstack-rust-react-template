---
name: frontend-react-authorization
description: How to gate UI and routes by role (RBAC) or per-resource policy (PBAC) using lib/authorization and ProtectedRoute. Use this when adding or updating access control, role checks, permission gates, or protected routes in the frontend.
---

# Authorization

Access control lives in `src/lib/authorization.tsx` (RBAC + PBAC) and `src/lib/auth.tsx`
(`ProtectedRoute`). **This is UX only — always enforce on the server too.**

## Roles (RBAC) and policies (PBAC)

```tsx
export enum ROLES { ADMIN = 'ADMIN', USER = 'USER' }

export const POLICIES = {
    'comment:delete': (user: User, comment: Comment) =>
        user.role === 'ADMIN' || (user.role === 'USER' && comment.author?.id === user.id),
}
```

- **RBAC** = coarse role check (`ADMIN` vs `USER`).
- **PBAC** = per-resource check; add a `'<resource>:<action>'` function to `POLICIES`.

## Gate UI with `<Authorization>`

Role-based:
```tsx
import { Authorization, ROLES } from '@/lib/authorization'

<Authorization allowedRoles={[ROLES.ADMIN]}>
    <DeleteThing id={id} />
</Authorization>
```

Policy-based:
```tsx
import { Authorization, POLICIES, useAuthorization } from '@/lib/authorization'

const ConfirmDelete = ({ comment }) => {
    const user = useUser()
    return (
        <Authorization policyCheck={POLICIES['comment:delete'](user.data!, comment)}>
            <DeleteComment id={comment.id} />
        </Authorization>
    )
}
```
`<Authorization>` accepts **either** `allowedRoles` **or** `policyCheck` (not both), plus an optional `forbiddenFallback`.

## Imperative check

```tsx
const { checkAccess, role } = useAuthorization()
if (checkAccess({ allowedRoles: [ROLES.ADMIN] })) { /* ... */ }
```

## Protect a route

Authenticated routes are already wrapped at `app/router.tsx`:
```tsx
{ path: paths.app.root.path, element: <ProtectedRoute><AppRoot /></ProtectedRoute>, children: [...] }
```
`ProtectedRoute` (from `@/lib/auth`) redirects unauthenticated users to login, preserving `redirectTo`. New `/app` children inherit this — no extra wrapping needed.

## Rules

- **New role** → add to the `ROLES` enum. **New permission** → add a function to `POLICIES`. Don't scatter `user.role === 'ADMIN'` checks across components.
- **Gate the action UI** (the create/delete button) with `<Authorization>` — see `delete-discussion.tsx` / `create-discussion.tsx`.
- `useAuthorization()` throws if no user — only call it inside authenticated screens.
- Server-side checks are the real boundary; client gating only hides UI.
