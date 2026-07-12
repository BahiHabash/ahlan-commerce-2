# Chapter 03 Observability Contract

Use `tracing` for structured diagnostics.

## Request Fields

Every API request should produce a span or structured event with:

- `request_id`
- `method`
- `route`
- `status`
- `latency_ms`
- `error_code` when the request fails

## Domain And Infrastructure Fields

Add these only when the code path owns them:

- `product_id`
- `product_handle`
- `job_id`
- `cache_key`
- `operation`

## Rules

- Do not log secrets, access tokens, full connection strings, or raw customer
  payloads.
- Log enough identifiers to join API errors, worker events, cache behavior, and
  database failures.
- Use `tower-http` request tracing when the HTTP server is ready for middleware.
- Use `tracing-subscriber` at application startup to configure filters and log
  output.
- Use explicit error categories from [error-contract.md](error-contract.md)
  instead of free-form error labels.

## Why

Tests prove behavior before release. Tracing explains behavior after release.
The goal is not noisy logs; the goal is enough stable context to debug a real
production incident.
