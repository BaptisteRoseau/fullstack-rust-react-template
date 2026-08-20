# GET /auth/register

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/auth/register` |
| **Full URL** | `/api/auth/register` |
| **Auth** | OIDC; API Key |

## Input

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `redirect` | array[string, null] | No | Same-origin path to return to after a successful login. |

## Response 303

Redirect to the login or registration page.

