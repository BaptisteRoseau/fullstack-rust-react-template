# Authentication

This project authenticates users against **Keycloak** using a **Backend-for-Frontend
(BFF)** pattern: the React app never handles tokens itself. Instead, the Rust backend
drives the OAuth **Authorization Code + PKCE** flow, stores the access and refresh tokens
in **httpOnly cookies**, and exposes a small set of endpoints to the SPA. Access tokens are
short-lived and refreshed transparently when they expire.

## Why this design

| Requirement | How it is met |
| --- | --- |
| Register / log in out of the box | Keycloak's hosted pages with self-service registration enabled; the realm is provisioned automatically on startup. |
| Tokens not reachable by JavaScript | Tokens live in `HttpOnly` cookies set by the backend; the SPA can neither read nor set them. |
| Short-lived access tokens | A deliberately short access-token lifespan in the realm. |
| Silent refresh | On a `401 TokenExpired`, the frontend calls `/auth/refresh` once and replays the request — the user notices nothing. |
| Backend stays a stateless resource server | It keeps validating JWTs via JWKS; the BFF endpoints are an additive layer. |

## Documentation map

| File | Contents |
| --- | --- |
| [overview.md](./overview.md) | The end-to-end flow and sequence diagram. Start here. |
| [keycloak.md](./keycloak.md) | Realm, clients, and how the realm is bootstrapped. |
| [backend.md](./backend.md) | The Rust side: the `Authenticator` trait, the `/auth/*` endpoints, cookies, the extractor, error mapping. |
| [frontend.md](./frontend.md) | The React side: redirect-based login, `useUser`, and the refresh-and-retry interceptor. |
| [configuration.md](./configuration.md) | Every environment variable / setting, backend and frontend. |
| [manual-testing.md](./manual-testing.md) | Run the flow end to end and obtain a raw JWT for `curl`. |

## TL;DR — run it locally

```bash
# 1. Start Keycloak + supporting services (the `app` realm is imported automatically)
docker compose up -d

# 2. Run the backend (defaults already point at the local Keycloak)
cargo run -p backend

# 3. Run the frontend against the real backend (mocks off)
cd frontend && VITE_APP_ENABLE_API_MOCKING=false bun run dev
```

Open the app, hit a protected route, and you are redirected to Keycloak to register or log
in. See [manual-testing.md](./manual-testing.md) for the full walkthrough.
