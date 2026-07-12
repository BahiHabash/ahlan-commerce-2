# Chapter 14 OpenAPI Contract

Use `utoipa`, `utoipa-axum`, and `utoipa-scalar` for REST documentation when the
REST surface is ready to document.

## Required Output

- `docs/generated/openapi.json` for health, product REST, and import-job REST
  endpoints.
- `docs/generated/schema.graphql` for the GraphQL schema.
- `/docs/scalar` local route for viewing REST docs.
- `make docs-api` command that regenerates both generated artifacts.
- `make docs-api-check` command that fails when generated artifacts are stale.

## Rules

- Generated docs show the current API shape.
- Written docs explain setup, tradeoffs, operations, and architecture.
- Generated REST docs and exported GraphQL schema are separate artifacts.
- Do not hand-edit generated OpenAPI output.
- `docs/api.md` may summarize links and commands, but it must not replace the
  generated artifacts.
- CI must run `make docs-api-check`.

## Done Check

A mentor can run `make docs-api`, run `make docs-api-check`, open
`/docs/scalar`, and inspect `docs/generated/schema.graphql` without reading
handler or resolver code.
