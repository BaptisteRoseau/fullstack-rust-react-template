# GET /auth/me

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/auth/me` |
| **Full URL** | `/api/auth/me` |
| **Auth** | OIDC; API Key |

## Response 200

**Response Content-Type:** `application/json`

The authenticated user's profile.

```jsonc
{
  "bio": "string",  // string, required
  "createdAt": 0,  // integer (int64), required
  "email": "string",  // string, required
  "firstName": "string",  // string, required
  "id": "string",  // string, required
  "lastName": "string",  // string, required
  "role": "string",  // string, required
  "teamId": "string"  // string, required
}
```

## Response 401

**Response Content-Type:** `application/json`

Not authenticated.

See [ApiErrorResponse](../../schemas/api-error-response.md)

