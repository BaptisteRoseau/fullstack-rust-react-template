# PUT /storage/upload/{file}

| | |
|--|--|
| **Method** | `PUT` |
| **URL** | `/storage/upload/{file}` |
| **Full URL** | `/api/storage/upload/{file}` |
| **Auth** | OIDC; API Key |
| **Request Content-Type** | `application/octet-stream` |

## Input

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file` | string | Yes | The file path/name to store |

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `compression` | array[boolean, null] | No | Enable gzip compression. Defaults to true. |
| `imageCompression` | array[string, null] | No | Image compression mode: "none", "lossless", "lossy", "auto". Defaults to "lossy". |
| `imageConversion` | array[string, null] | No | Image conversion format: "none", "webp", "jpeg", "png", "tiff". Defaults to no conversion. |
| `imageHeight` | array[integer, null] (≥0) | No | Desired image height for resizing. |
| `imageWidth` | array[integer, null] (≥0) | No | Desired image width for resizing. |

### Payload

```jsonc
"string"  // string
```

## Response 200

**Response Content-Type:** `application/json`

File has been successfully uploaded.

```jsonc
{
  "file": "string"  // string, required
}
```

## Response 500

**Response Content-Type:** `application/json`

Storage error.

See [ApiErrorResponse](../../schemas/api-error-response.md)

