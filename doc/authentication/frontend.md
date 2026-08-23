# Authentication — Frontend

The React app never handles a token. It starts login with a full-page redirect to the backend,
reads the signed-in user from `/api/auth/me`, and lets the browser attach the httpOnly cookies on
its own.

## Where the code lives

| Concern | Code |
| --- | --- |
| Login and register URLs, logout | [api/domains/session](../../frontend/src/api/domains/session) |
| The signed-in user | [api/domains/currentUser](../../frontend/src/api/domains/currentUser) |
| SWR bindings | [api/hooks/useApiCurrentUser](../../frontend/src/api/hooks/useApiCurrentUser), [api/hooks/useApiLogout](../../frontend/src/api/hooks/useApiLogout) |
| Cookies and the retry after a 401 | [api/client.ts](../../frontend/src/api/client.ts) |
| Route guard | [components/ProtectedRoute](../../frontend/src/components/ProtectedRoute) |
| The two redirect pages | [pages/Login](../../frontend/src/pages/Login), [pages/Register](../../frontend/src/pages/Register) |

## Signing in, step by step

1. The user opens a protected route. `ProtectedRoute` calls `useApiCurrentUser` and renders a
   spinner while the session resolves.
2. The hook resolves to `null`. `ProtectedRoute` navigates to the login page, putting the route the
   user wanted in `?redirect=`.
3. The login page is **one button**, not a form. The credential UI belongs to Keycloak.
4. The button sets `location.href` to `loginUrl(redirect)`. That URL points at the **backend**, not
   at Keycloak. It is a browser navigation, not a fetch, so nothing is awaited.
5. The backend redirects the browser to Keycloak, the user signs in, and Keycloak sends the browser
   back to the backend callback. The callback sets the cookies and redirects into the app. See
   [backend.md](./backend.md).
6. `useApiCurrentUser` revalidates, now returns a user, and the route renders.

Registration is the same flow through `registerUrl`.

## Reading the signed-in user

`fetchCurrentUser` turns a `401` into `null` instead of throwing. Being signed out is an answer, not
a failure, so callers render the signed-out interface without a special case.

Everything else in the app treats a failed request as an error, as usual.

## Silent refresh

`fetchWithSessionRefresh` wraps `fetch` and is installed on the generated SDK client. It wraps
`fetch` rather than using the SDK's interceptors so it stays independent of the code generator.

1. A request comes back `401`.
2. The wrapper calls `POST /api/auth/refresh` **once**. Concurrent 401s share one in-flight
   promise, so an expired session produces a single refresh.
3. On success, the original request is replayed from a clone taken **before** the first send. A
   request body is a stream and cannot be read twice.
4. On failure, the original `401` is returned. `fetchCurrentUser` maps it to `null` and
   `ProtectedRoute` sends the user to the login page.

The refresh URL is excluded from the retry, so a dead session cannot loop.

## Signing out

`useApiLogout` calls `POST /api/auth/logout`. The backend revokes the session at Keycloak and
clears the cookies.

## Mocks

[test-utils/mocks/handlers/auth.ts](../../frontend/src/test-utils/mocks/handlers/auth.ts) mirrors
the real contract so tests run with no backend: the login and register endpoints answer with a
redirect and a session cookie, and `/api/auth/me` returns the user or `401`. Keycloak is not
involved.

The MSW **browser** worker intercepts XHR but not full-page navigations. The OIDC redirect is
therefore only mocked when the app points at the standalone mock server (`bun run run-mock-server`),
which is what the Playwright suite does. See Skill(frontend-mocks).

## Configuration

The frontend needs **no** Keycloak settings — the backend hides them. See
[configuration.md](./configuration.md).
