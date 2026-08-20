# GET /auth/login

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/auth/login` |
| **Full URL** | `/api/auth/login` |
| **Auth** | OIDC; API Key |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `redirect` | array[string, null] | No | Same-origin path to return to after a successful login. |

## Response 303

Redirect to the login page.

