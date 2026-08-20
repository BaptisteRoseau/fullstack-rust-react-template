# GET /auth/callback

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/auth/callback` |
| **Full URL** | `/api/auth/callback` |
| **Auth** | OIDC; API Key |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `code` | array[string, null] | No | Authorization code issued by Keycloak. |
| `state` | array[string, null] | No | CSRF state echoed back by Keycloak. |

## Response 303

Tokens stored in httpOnly cookies; redirect to the frontend.

