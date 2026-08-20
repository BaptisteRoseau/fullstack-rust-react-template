# POST /auth/refresh

| | |
|--|--|
| **Method** | `POST` |
| **URL** | `/auth/refresh` |
| **Full URL** | `/api/auth/refresh` |
| **Auth** | OIDC; API Key |

## Response 200

Access token refreshed; cookies updated.

## Response 401

No valid refresh token; the user must log in again.

