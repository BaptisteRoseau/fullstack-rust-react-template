# `features/auth`

Authentication UI. **Note the split:** the auth *logic* — fetchers, Zod schemas, and `configureAuth`
(producing `useUser`/`useLogin`/`useLogout`/`useRegister`/`AuthLoader`) plus `ProtectedRoute` — lives
in `@/lib/auth.tsx` because it is shared across features. This folder holds only the **form components**.

```
auth/
└── components/   # login-form, register-form (+ __tests__)
```

The forms use the shared `@/components/ui/form` primitives with the `loginInputSchema` /
`registerInputSchema` from `@/lib/auth`, and call `useLogin` / `useRegister`.

See `.claude/skills/frontend-react-authorization` (roles/route protection) and
`.claude/skills/frontend-react-form` (the form pattern).
