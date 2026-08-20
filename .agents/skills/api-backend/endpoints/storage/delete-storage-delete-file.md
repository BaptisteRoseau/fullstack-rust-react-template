# DELETE /storage/delete/{file}

| | |
|--|--|
| **Method** | `DELETE` |
| **URL** | `/storage/delete/{file}` |
| **Full URL** | `/api/storage/delete/{file}` |
| **Auth** | OIDC; API Key |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file` | string | Yes | The file path/name to delete |

## Response 200

File successfully deleted.

## Response 500

**Response Content-Type:** `application/json`

Storage error.

See [ApiErrorResponse](../../schemas/api-error-response.md)

