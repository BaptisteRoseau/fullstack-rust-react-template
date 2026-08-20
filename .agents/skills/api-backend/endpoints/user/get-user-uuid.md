# GET /user/{uuid}

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/user/{uuid}` |
| **Full URL** | `/api/user/{uuid}` |
| **Auth** | OIDC; API Key |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `uuid` | string (uuid) | Yes | - |

## Response 200

**Response Content-Type:** `application/json`

The user information.

```jsonc
{
  "name": "string"  // string, required
}
```

## Response 404

**Response Content-Type:** `application/json`

The user does not exist.

See [ApiErrorResponse](../../schemas/api-error-response.md)

