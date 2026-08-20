# ApiErrorResponse

This is the standard API error returned by endpoints.

```jsonc
{
  "error": "string",  // string, required
  "id": "UNEXPECTED"  // string, required, enum: "UNEXPECTED", "UNAUTHORIZED", "FORBIDDEN", "TOKEN_EXPIRED", "NOT_FOUND", "TOO_MANY_REQUESTS", "HEADER_INVALID_ASCII_CHARACTERS", An enum representing and API error.
}
```
