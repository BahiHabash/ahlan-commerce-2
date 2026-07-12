# Chapter 03A - Error Handling With rootcause

## Why Now

Your API has handlers, DTOs, and domain functions. Before adding a database,
make failure behavior explicit. Once Postgres, Redis, workers, GraphQL, and
deployment arrive, unclear errors become much harder to debug.

## Learn First

- [Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
- [`thiserror`](https://docs.rs/thiserror/latest/thiserror/)
- [`anyhow`](https://docs.rs/anyhow/latest/anyhow/)
- [`rootcause`](https://docs.rs/rootcause/latest/rootcause/)

## What Is Common In Rust

Rust services commonly use a mix of:

- typed error enums for expected application failures
- `thiserror` to remove boilerplate from those enums
- `anyhow` or `eyre` in binaries, scripts, or boundaries where exact error type
  is less important
- `IntoResponse` mapping in Axum so application errors become HTTP responses
- context/root-cause capture so logs explain what really failed

The common mistake is returning raw internal errors to clients or losing the
real source error while converting everything into strings.

## Alternatives

### String Errors

Pros:

- Fast to write.

Cons:

- No stable error codes.
- No exhaustiveness.
- Easy to leak internal details.

### `thiserror`

Pros:

- Strong typed error enums.
- Good for domain and application errors.
- Common in Rust libraries.

Cons:

- Does not by itself preserve high-level context across boundaries.

### `anyhow` / `eyre`

Pros:

- Very ergonomic.
- Good for binaries and operational tools.

Cons:

- Less precise as an API contract.
- Easy to hide which failures clients should handle.

### `miette` / `error-stack` / `snafu`

Pros:

- Rich diagnostics and context patterns.

Cons:

- More concepts than Ahlan needs at this stage.

### `rootcause`

Pros:

- Keeps the root failure visible to engineers.
- Fits service code where public errors must stay safe but internal causes must
  remain debuggable.
- Small enough for onboarding.

Cons:

- Still needs a typed application error contract around it.
- Engineers must avoid treating root-cause text as a public API.

## Why We Choose This

Use a typed application error enum for public behavior and `rootcause` for
internal context. The enum owns client-facing `code`, HTTP status, and safe
message. `rootcause` helps preserve the underlying cause for logs and debugging.

```text
source error -> root-cause context -> app error -> HTTP/GraphQL error contract
```

This mirrors the service mindset needed in Majaz: user-facing errors are stable
contracts; internal causes are for engineers.

## Chapter 03A Tasks

### Task 03A.1 - Create The Public Error Contract

Input:

- Chapter 03 handlers and DTOs

Output:

- error response shape based on
  [../03-in-memory-product-api/error-contract.md](../03-in-memory-product-api/error-contract.md)
- typed app error enum
- mapping from app error to HTTP status and public error code

Done when:

- validation failure returns `400` and `validation_failed`
- duplicate handle returns `409` and `duplicate_product_handle`
- not found returns `404` and `not_found`
- no handler returns ad hoc error strings

### Task 03A.2 - Preserve Internal Causes

Input:

- app error enum
- one simulated internal failure

Output:

- root-cause context attached to internal failures
- logs include the internal cause
- captured root-cause log example in `docs/logs/root-cause-error.log`
- response body still uses safe public message

Done when:

- mentor can see the internal cause in logs
- `docs/logs/root-cause-error.log` shows the safe public error code and the
  internal root cause
- client response does not expose database, file path, Redis, or stack details

### Task 03A.3 - Test Error Mapping

Input:

- error contract and handler mapping

Output:

- tests for validation, duplicate handle, not found, and internal error mapping
- tests assert status code, error code, and response shape

Done when:

- tests prove public behavior without depending on internal error text

## Do Not Add Yet

- Postgres-specific errors
- Redis errors
- worker failures
- GraphQL error extensions
- rich diagnostic UI

## Done When

- You can explain typed errors vs root-cause context.
- You can explain why public errors are stable contracts.
- You can explain why raw internal errors must not leak to clients.

Next: [Tracing And Observability](../03b-tracing-observability/README.md)
