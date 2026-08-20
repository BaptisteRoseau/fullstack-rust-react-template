# DELETE /api-key/{id}

| | |
|--|--|
| **Method** | `DELETE` |
| **URL** | `/api-key/{id}` |
| **Full URL** | `/api/api-key/{id}` |
| **Auth** | OIDC; API Key |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string (uuid) | Yes | API key ID |

## Response 204

API key deleted.

## Response 401

**Response Content-Type:** `application/json`

Not authenticated.

See [ApiErrorResponse](../../schemas/api-error-response.md)

## Response 404

Not found or not owned by caller.

