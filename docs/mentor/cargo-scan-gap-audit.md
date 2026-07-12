# Cargo Scan Gap Audit

This file records topics discovered from workspace `Cargo.toml` files that must
either appear in Ahlan Commerce or be deliberately left out.

## Added To The Book

| Cargo topic | Book coverage |
| --- | --- |
| `rootcause` | Chapter 03A first-class error handling chapter and final review questions |
| `thiserror` | Chapter 03A as the common typed error-enum companion |
| `tracing` | Chapter 03B first-class observability chapter and final review questions |
| `tracing-subscriber` | Chapter 03B as startup logging/filter setup |
| `tracing-appender` | Covered conceptually by runtime logging; keep nonblocking/file logging as a mentor extension |
| `tower-http` tracing layers | Chapter 03B observability contract |
| `uuid` with UUIDv7 usage | Chapter 03 ID/time contract and SQL schema comments |
| `async-graphql` | Chapter 09 GraphQL slice |
| `async-graphql-axum` | Chapter 09 GraphQL slice |
| `utoipa` | Chapter 14 OpenAPI contract |
| `utoipa-axum` | Chapter 14 OpenAPI contract |
| `utoipa-scalar` | Chapter 14 OpenAPI contract |
| `redis` / `bb8-redis` | Chapter 12 Redis cache |
| `cornucopia` generated query bindings | Chapter 07 SQL-first DAL |

## Deliberately Smaller In Ahlan

| Cargo topic | Decision |
| --- | --- |
| `object_store` | Not in the default Ahlan path. Add only as a mentor extension for file/object storage. |
| `reqwest` | Not in the default Ahlan path. Add only if teaching outbound HTTP clients or external sync. |
| `csv`, `zip`, `tar` | Not in the default import worker. Add only if the mentor wants a richer import-file exercise. |
| `mayya`, `liquid`, theme crates | Excluded from Ahlan. The storefront chapter teaches rendering boundaries without private theme-engine complexity. |
| Apollo Client | Optional mentor extension in Chapter 10 only. The required path uses TanStack Query plus a thin GraphQL wrapper. |
| shadcn UI primitives | Not in the default Ahlan path. Add only after the learner understands UI state and data flow. |

## Mentor Rule

If one of these smaller-path decisions becomes important for a candidate's
role, add a focused extension chapter rather than mixing it into earlier tasks.
