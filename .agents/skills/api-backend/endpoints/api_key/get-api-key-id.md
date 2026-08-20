# GET /api-key/{id}

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/api-key/{id}` |
| **Full URL** | `/api/api-key/{id}` |
| **Auth** | OIDC; API Key |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string (uuid) | Yes | API key ID |

## Response 200

**Response Content-Type:** `application/json`

API key metadata.

```jsonc
{
  "createdAt": "string",  // string, format: date-time, required
  "id": "string",  // string, format: uuid, required
  "name": "string",  // string, required
  "permissions": [  // array of string, required
    "string"
  ]
}
```

## Response 401

**Response Content-Type:** `application/json`

Not authenticated.

See [ApiErrorResponse](../../schemas/api-error-response.md)

## Response 404

Not found or not owned by caller.

