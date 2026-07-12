# Chapter 02 - Axum Basics

## Why Now

You have product logic. Now you need to expose it over HTTP.

## Learn First

- [Axum documentation](https://docs.rs/axum/latest/axum/)
- [Axum examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)

## What Axum Does

Axum lets you define routes, handlers, extract request data, and return
responses using Tokio async Rust.

## Alternatives

### Actix Web

Pros:

- Mature and fast.
- Large ecosystem.

Cons:

- Different ecosystem style than Tokio/Tower-first services.

### Rocket

Pros:

- Very ergonomic.
- Good beginner experience.

Cons:

- Less aligned with Tower middleware patterns.

### Warp

Pros:

- Powerful filter model.

Cons:

- Can be harder to read for beginners.

Why Axum for us:

- Axum is Tokio/Tower-friendly, explicit, and a good fit for service code where
  handlers should stay thin.

## Why We Choose This

We choose Axum because it teaches the same service stack engineers need around
Majaz: Tokio for async execution, Hyper for HTTP, Tower for middleware, and Axum
for routing/extraction.

## What Engineers Should Notice

Axum usually runs inside one Tokio runtime, commonly the multi-thread runtime
from `#[tokio::main]`. Requests are served through Hyper and Tower, so the app is
part of this stack:

```text
Tokio runtime -> Hyper server -> Tower layers -> Axum router -> handler
```

This differs from Actix Web. Actix Web also runs on Tokio, but each server worker
uses a single-threaded runtime, and the app factory is instantiated per worker.
That changes how per-worker state, shared state, blocking work, and spawned tasks
behave.

For Ahlan and Majaz-style services, the important lesson is not only route
syntax. You should understand:

- one shared `AppState` should be cloned into the router intentionally
- request handlers should stay thin
- middleware is usually Tower layers
- blocking work must not run directly on async request tasks
- framework state is not a place for hidden business rules

## Chapter 02 Tasks

### Task 02.1 - Start An Axum Server

Input:

- product domain code from chapter 01

Output:

- Axum server using Tokio
- shared `AppState`
- `GET /health`

Expected API:

See [api-contract.md](api-contract.md).

Done when:

- `curl http://localhost:3000/health` returns the expected JSON

### Task 02.2 - Expose Product Routes

Input:

- running Axum server
- in-memory product behavior

Output:

- `GET /api/products`
- `POST /api/products`
- handlers that call existing product behavior

Expected API:

See [api-contract.md](api-contract.md).

Done when:

- a product can be created with `curl`
- a following list request returns that product

### Task 02.3 - Explain The Runtime Model

Input:

- working Axum server

Output:

- `docs/runtime-notes.md`
- short note comparing Axum's Tokio/Hyper/Tower model with Actix Web worker
  runtimes

Done when:

- you can explain where handlers run and why blocking work is risky

## Do Not Add Yet

- database
- GraphQL
- Makefile
- mprocs
- route constants
- central config

## Done When

- `curl /health` works.
- `GET /api/products` returns JSON.
- `POST /api/products` creates an in-memory product.
- You can explain route vs handler.

Next: [In-Memory Product API](../03-in-memory-product-api/README.md)
