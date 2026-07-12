# Chapter 03 Error Contract

Use one public error envelope for API failures.

```json
{
  "error": {
    "code": "validation_failed",
    "message": "Product title is required.",
    "request_id": "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001"
  }
}
```

## Required Codes

| Code | HTTP status | Public meaning |
| --- | --- | --- |
| `validation_failed` | `400` | The request body or parameters are invalid. |
| `duplicate_product_handle` | `409` | Another product already uses this handle. |
| `not_found` | `404` | The requested resource does not exist. |
| `dependency_unavailable` | `503` | A required dependency such as Postgres or Redis is unavailable. |
| `internal_error` | `500` | The server failed unexpectedly. |

## Rules

- Public messages must be safe for users and logs.
- Internal database, Redis, worker, parser, or filesystem details must not be
  returned in the response body.
- Internal causes must be retained with `rootcause` context so an engineer can
  debug the real failure without exposing private details to clients.
- Tracing logs must include the request ID and safe public error code for failed
  requests.
- `thiserror` may be used for a typed Rust error enum when it keeps status/code
  mapping explicit.
- Every error response must include the request ID.
- Tests should assert the status code, error code, and response shape.

## Why

Ad hoc error strings are easy at first, but they break clients and make
production debugging inconsistent. A small envelope gives the frontend,
tests, logs, and mentors one shared contract.
