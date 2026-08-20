# Authentication — Frontend

The React app never handles tokens. It starts the login flow with a full-page redirect to the
backend, reads the current user from `/api/auth/me`, and relies on the browser to attach the
httpOnly cookies automatically.

## Auth module

Endpoint declarations: [`frontend/src/api/auth.ts`](../../frontend/src/api/auth.ts)

- `ME_ENDPOINT`, `LOGOUT_ENDPOINT`, `REFRESH_ENDPOINT` — the paths.
- `authRedirectUrl(screen, redirectTo?)` — builds the BFF entry point
  (`${API_URL}/api/auth/login` or `/api/auth/register`, with `?redirect=` to return to a route).
  Navigating the browser there starts the OAuth flow.
- `CurrentUser`, `UpdateProfileBody`, `fullName(user)`.

Service hooks: [`frontend/src/api/service/auth.ts`](../../frontend/src/api/service/auth.ts)

- `useCurrentUser()` — SWR hook backed by `GET /api/auth/me`. It uses a dedicated fetcher that
  turns a `401` into `data: null` rather than an error, because being logged out is a normal state,
  not a failure.
- `useUpdateProfile()` — `PATCH /api/auth/me`.
- `useLogout()` — `POST /api/auth/logout`.

Route guard: [`frontend/src/components/ProtectedRoute`](../../frontend/src/components/ProtectedRoute)
renders a spinner while the session resolves, then redirects to `/auth/login?redirect=<pathname>`
when there is no user.

Because login and registration are full-page redirects (the credential UI lives on Keycloak), the
login and register **pages are a single button**, not input forms:

- [`pages/Login`](../../frontend/src/pages/Login) — "Continue to sign in".
- [`pages/Register`](../../frontend/src/pages/Register) — "Continue to registration".

## Silent refresh — the fetch wrapper

[`frontend/src/api/client.ts`](../../frontend/src/api/client.ts)

- Every request uses `credentials: 'include'`, so the cookies are sent automatically.
- On a `401`, `apiFetch` calls `POST /api/auth/refresh` **once** (concurrent 401s share a single
  in-flight promise) and replays the original request if the refresh succeeded.
- `/api/auth/refresh` is excluded from the retry, which prevents an infinite loop.
- A `401` raises no toast — being logged out is an expected signal, surfaced through routing.

```
request ──▶ 401 ──▶ POST /api/auth/refresh ──┬─ ok  ─▶ replay original request ─▶ success
                                              └─ 401 ─▶ throw ApiError ─▶ ProtectedRoute ─▶ login
```

## Configuration

The frontend only needs the backend origin — no Keycloak settings, since the BFF hides them. See
[configuration.md](./configuration.md).

- `VITE_APP_API_URL` — the bare origin, e.g. `http://localhost:8080`. Endpoint paths in `src/api/*`
  already include the `/api` prefix.
- `VITE_APP_ENABLE_API_MOCKING` — `false` to use the real backend, `true` to start the MSW browser
  worker.

## Mocks and tests

[`frontend/src/test-utils/mocks/handlers/auth.ts`](../../frontend/src/test-utils/mocks/handlers/auth.ts)
mirrors the real contract so tests run without a backend: `/api/auth/{login,register}` answer `303`
with a `Set-Cookie` and a `Location` back into the app — the same shape as the real redirect — and
`/api/auth/me` returns the user or `401`. The mock flow is independent of Keycloak.

The MSW **browser** worker intercepts XHR but not full-page navigations, so the OIDC redirect is
only mocked when the app points at the standalone mock server (`npm run run-mock-server`), which is
what the Playwright suite does.
