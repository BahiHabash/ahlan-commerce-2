# Chapter 04 - PostgreSQL Basics

## Why Now

In-memory products disappear when the server restarts. You now need persistence.
This chapter intentionally starts with plain SQL so you feel what works locally
and what becomes unsafe when schema changes are done by memory.

## Learn First

- [PostgreSQL tutorial](https://www.postgresql.org/docs/current/tutorial.html)
- [Atlas documentation](https://atlasgo.io/docs)
- [Docker overview](https://docs.docker.com/get-started/overview/)

## Alternatives

### SQLite

Pros:

- Very easy locally.
- No separate server.

Cons:

- Does not teach service/database runtime separation.

### PostgreSQL With Plain SQL

Pros:

- Direct and understandable.
- Good first step for learning tables, rows, and connection pools.

Cons:

- Schema changes are easy to forget.
- Raw SQL in app code can drift from Rust structs.

### Manual Schema Changes

Pros:

- Fast during local experiments.

Cons:

- Fragile and unsafe for production.
- Easy to forget which change was applied where.
- Recreating a database loses data and does not model production.

### Atlas Migrations

Pros:

- Makes schema history explicit.
- Gives repeatable commands for local and hosted databases.
- Makes review of schema changes possible.

Cons:

- Adds a migration workflow before generated queries exist.

Why PostgreSQL for us:

- You need to practice service plus database development before learning the
  workflow tools that make multiple processes manageable.

## Why We Choose This

We choose PostgreSQL because Majaz relies on a real service database, not local
embedded storage. We first use plain SQL because it is the clearest way to learn
the boundary. We introduce Atlas only after you feel why manual schema changes
are too fragile.

## What Engineers Should Notice

PostgreSQL is not an in-process data structure. It is a separate service with
connections, transactions, schema changes, startup order, and failure modes.

The important progression is:

```text
in-memory data -> plain SQL persistence -> painful schema change -> Atlas migration history
```

Atlas solves schema migration discipline. It does not make Rust queries typed,
and it does not decide where persistence logic belongs in the application.

## Chapter 04 Tasks

### Task 04.1 - Persist With Plain SQL

Input:

- in-memory product API from Chapter 03

Output:

- local PostgreSQL database
- database named `ahlan_commerce`
- `products` table using [initial-products.sql](initial-products.sql)
- app reads/writes products with plain SQL
- connection pool created at startup and passed through state
- no Atlas
- no query code generation

Done when:

- restarting the API does not lose products
- `POST /api/products` and `GET /api/products` still match
  [Chapter 02 API contract](../02-axum-basics/api-contract.md)
- you can explain what the connection pool does
- you can explain why plain SQL worked for the first version

### Task 04.2 - Change The Schema The Painful Way

Input:

- working plain SQL persistence

Change:

- add `description text`
- add `published_at timestamptz`

Output:

- manual schema change notes in `docs/manual-schema-change.md`
- app updated to read/write `description`
- app updated to set/read `published_at` for published products
- REST product API updated to match
  [product-api-contract.md](product-api-contract.md)
- reflection answering:
  - What did you have to remember?
  - What broke when Rust code and DB schema disagreed?
  - Why would this be dangerous in production?

Done when:

- `POST /api/products` accepts `description`
- `GET /api/products` returns `description`, `published_at`, `created_at`, and
  `updated_at`
- you can explain why dropping/recreating a database is not a production migration
- you can explain why manually editing production schema is unsafe

### Task 04.3 - Introduce Atlas

Input:

- manual schema-change pain from Task 04.2

Output:

- `atlas.hcl`
- `db/schema/products.sql` based on [products.sql](products.sql)
- generated migration under `db/migrations/`
- Atlas command notes based on [atlas-contract.md](atlas-contract.md)

Done when:

- `atlas migrate apply --env local` can recreate the expected schema
- you can explain schema file vs migration file
- you can explain why migration history matters

### Task 04.4 - Keep Plain SQL For Now

Input:

- Atlas-managed schema

Output:

- app still uses plain SQL temporarily
- short note in `docs/manual-schema-change.md` explaining that Atlas solved
  schema-change discipline, not Rust query safety

Done when:

- you understand why the next pain is query/code drift

## Do Not Add Yet

- Makefile
- mprocs
- query code generation
- Redis

## Done When

- Products persist in PostgreSQL.
- Product REST API matches [product-api-contract.md](product-api-contract.md).
- You can explain the connection pool.
- You can explain manual schema change pain.
- You can explain what Atlas solves and what it does not solve.

Next: [Make For Repeated Commands](../05-make-for-repeated-commands/README.md)
