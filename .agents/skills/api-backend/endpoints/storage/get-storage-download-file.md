# GET /storage/download/{file}

| | |
|--|--|
| **Method** | `GET` |
| **URL** | `/storage/download/{file}` |
| **Full URL** | `/api/storage/download/{file}` |
| **Auth** | OIDC; API Key |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file` | string | Yes | The file path/name to retrieve |

## Response 200

**Response Content-Type:** `application/octet-stream`

File successfully downloaded.

## Response 500

**Response Content-Type:** `application/json`

Storage error.

See [ApiErrorResponse](../../schemas/api-error-response.md)

