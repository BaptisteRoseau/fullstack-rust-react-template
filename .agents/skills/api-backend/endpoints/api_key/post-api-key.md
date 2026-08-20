# POST /api-key

| | |
|--|--|
| **Method** | `POST` |
| **URL** | `/api-key` |
| **Full URL** | `/api/api-key` |
| **Auth** | OIDC; API Key |
| **Request Content-Type** | `application/json` |

## Input

### Payload

```jsonc
{
  "name": "string",  // string, required
  "permissions": [  // array of string, required
    "string"
  ]
}
```

## Response 201

**Response Content-Type:** `application/json`

API key created.

```jsonc
{
  "createdAt": "string",  // string, format: date-time, required
  "id": "string",  // string, format: uuid, required
  "key": "string",  // string, required
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

