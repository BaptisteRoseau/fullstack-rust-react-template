# RBAC

Roles, scopes and permission checks. Answers one question: can this user do this thing to
this object?

This is not a service crate: it holds pure data and logic, no trait, no backend, no I/O.
[app_core](../app_core), [api](../api) and [models](../models) depend on it; it depends on
nothing in the workspace.

## Public surface

- [`Permissions`](src/permissions.rs) — the enum of actions the application knows about
  (e.g. `UploadFile`). New actions are added as new variants here.
- [`Scope`](src/scope.rs) — who an object is visible to: `Public`, `Users(HashSet<Uuid>)`,
  `Groups(HashSet<Uuid>)`, or `Mixed { users, groups, denied_users }`.
- [`AccessControl`](src/access_control.rs) — pairs an optional required-permissions set
  with a `Scope`. `AccessControl::has_access(&UserPermissions)` is the single check both
  conditions must pass.
- [`UserPermissions`](src/access_control.rs) — the caller's side of the check: user id,
  group ids and granted permissions.
- [`Role`](src/role.rs) — grants and forbids of `Permissions`, not yet wired into
  `has_access`.

## Directory

```txt
rbac/
├── src/
│   ├── permissions.rs      # the Permissions enum
│   ├── scope.rs            # Scope and its access check
│   ├── access_control.rs   # AccessControl, UserPermissions, has_access
│   └── role.rs             # Role (grants/forbids), not yet used elsewhere
└── Cargo.toml
```
