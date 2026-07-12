# Chapter 03B - Tracing And Observability

## Why Now

You have stable routes and stable errors. Now make runtime behavior visible
before adding Postgres, Redis, workers, cache, and deployment.

## Learn First

- [`tracing`](https://docs.rs/tracing/latest/tracing/)
- [`tracing-subscriber`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)
- [`tower-http` trace middleware](https://docs.rs/tower-http/latest/tower_http/trace/)
- Rust [`log`](https://docs.rs/log/latest/log/) crate basics

## What Is Common In Rust

Rust projects commonly use:

- `log` plus `env_logger` for simple application logs
- `tracing` for structured spans and events
- `tracing-subscriber` to configure formatting, filters, and output
- `tower-http` TraceLayer for HTTP request/response spans in Axum/Tower apps
- OpenTelemetry later when traces must leave the process and reach a collector

The common mistake is treating observability as `println!` debugging. Production
services need stable fields that let engineers connect requests, errors, jobs,
cache keys, and database failures.

## Alternatives

### `println!` / `dbg!`

Pros:

- Immediate and simple.

Cons:

- Not structured.
- Hard to filter.
- Easy to leave noisy or unsafe output.

### `log` + `env_logger`

Pros:

- Common and simple.
- Fine for small binaries.

Cons:

- No span model.
- Weaker for async request flows.

### `slog`

Pros:

- Structured logging model.

Cons:

- Less aligned with modern Tokio/Axum ecosystem than `tracing`.

### `tracing`

Pros:

- Structured events and spans.
- Works well with async Rust.
- Fits Axum/Tower services.
- Can grow toward OpenTelemetry later.

Cons:

- Requires discipline around fields and span boundaries.
- More concepts than plain logs.

## Why We Choose This

Use `tracing` because Ahlan is a service, not a script. Requests, workers,
cache operations, and deployment failures need stable diagnostic fields.

```text
request span -> handler event -> domain event -> dependency event -> response
```

Use `tracing-subscriber` at startup. Use `tower-http` TraceLayer for HTTP
request spans. Do not put tracing setup inside domain logic.

## Chapter 03B Tasks

### Task 03B.1 - Configure Tracing At Startup

Input:

- Chapter 03 Axum app
- Chapter 03A error contract

Output:

- `tracing-subscriber` setup at application startup
- env-controlled filter such as `RUST_LOG`
- logs formatted consistently for local development

Done when:

- startup logs show the app bind address
- changing the log filter changes emitted logs

### Task 03B.2 - Add HTTP Request Spans

Input:

- Axum router

Output:

- `tower-http` TraceLayer or equivalent Tower layer
- request span fields based on
  [../03-in-memory-product-api/observability-contract.md](../03-in-memory-product-api/observability-contract.md)
- captured success log example in `docs/logs/success-request.log`
- captured failure log example in `docs/logs/failed-request.log`

Required fields:

- `request_id`
- `method`
- `route`
- `status`
- `latency_ms`
- `error_code` when the request fails

Done when:

- one successful request logs route, status, and latency
- one failed request logs route, status, latency, and error code
- `docs/logs/success-request.log` contains a successful `GET /health` or
  `GET /api/products` request span
- `docs/logs/failed-request.log` contains a failed request span with
  `error_code`

### Task 03B.3 - Add Domain And Dependency Events

Input:

- product create/list behavior
- Chapter 03A error mapping

Output:

- product create event with `product_id` and `product_handle`
- validation failure event with safe error code
- no secrets or full request bodies in logs
- updated log examples showing the request span can be connected to the product
  event or validation failure event

Done when:

- mentor can trace one request from HTTP span to product event
- logs are useful without exposing sensitive values

## Do Not Add Yet

- OpenTelemetry collector
- metrics dashboard
- log shipping
- tracing-appender nonblocking file logging
- production alerting

## Done When

- You can explain logs vs spans vs events.
- You can explain why `tracing` fits async Axum services.
- You can explain which fields help debug production behavior.

Next: [PostgreSQL Basics](../04-postgresql-basics/README.md)
