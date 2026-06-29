# Authentication — Frontend

The React app never handles tokens. It starts the login flow with a full-page redirect to the
backend, reads the current user from `/auth/me`, and relies on the browser to attach the
httpOnly cookies automatically.

## Auth module

[`frontend/src/lib/auth.tsx`](../../frontend/src/lib/auth.tsx)

- `useUser()` — React Query hook backed by `GET /auth/me`; `user.data` is the logged-in user
  or `undefined`.
- `useLogout()` — `POST /auth/logout`.
- `loginUrl(redirectTo?)` / `registerUrl(redirectTo?)` — build the BFF entry-point URL
  (`${API_URL}/auth/login`, with `?screen=register` for sign-up and `?redirect=` to return to
  a route). Navigating the browser there starts the OAuth flow.
- `ProtectedRoute` — redirects to the login route when there is no user.

Because login and registration are full-page redirects (the credential UI lives on Keycloak),
the login/register **forms are plain links** rather than input forms:

- [`features/auth/components/login-form.tsx`](../../frontend/src/features/auth/components/login-form.tsx)
  — "Continue to sign in".
- [`features/auth/components/register-form.tsx`](../../frontend/src/features/auth/components/register-form.tsx)
  — "Create an account".

## Silent refresh — axios interceptor

[`frontend/src/lib/api-client.ts`](../../frontend/src/lib/api-client.ts)

- Every request uses `withCredentials: true`, so the cookies are sent automatically.
- A response interceptor handles `401`:
  1. call `POST /auth/refresh` **once** (concurrent 401s share a single in-flight promise),
  2. on success, **replay** the original request,
  3. on failure, reject and let `ProtectedRoute` route the user out.
- A per-request `_retry` flag prevents infinite loops, and `/auth/refresh` / `/auth/login` are
  excluded from the retry logic.
- `401` no longer raises an error toast — being logged out is an expected signal, surfaced via
  routing rather than a notification.

```
request ──▶ 401 ──▶ POST /auth/refresh ──┬─ 200 ─▶ replay original request ─▶ success
                                          └─ 401 ─▶ reject ─▶ ProtectedRoute ─▶ login
```

## Configuration

The frontend only needs to know the backend URL — no Keycloak settings, since the BFF hides
them. See [configuration.md](./configuration.md).

- `VITE_APP_API_URL` — e.g. `http://localhost:8080/api`.
- `VITE_APP_ENABLE_API_MOCKING` — set to `false` to use the real backend; `true` uses MSW.

## Mocks and tests

[`frontend/src/testing/mocks/handlers/auth.ts`](../../frontend/src/testing/mocks/handlers/auth.ts)
mirrors the real contract so tests run without a backend: `/auth/me` returns the bare user (or
`401` when logged out) and `/auth/refresh` reports "logged out". The mock flow is independent
of Keycloak.
