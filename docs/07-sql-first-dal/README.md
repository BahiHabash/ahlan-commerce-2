# Chapter 07 - SQL-First DAL

## Why Now

Atlas made schema changes repeatable, but your Rust code still owns raw SQL
strings. Now you need to feel query/code drift before adding code generation.

## Learn First

- [Cornucopia](https://github.com/cornucopia-rs/cornucopia)
- [Diesel guides](https://diesel.rs/guides/)
- [SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/)
- [SQLx `query!` macro](https://docs.rs/sqlx/latest/sqlx/macro.query.html)
- Review the SQL and schema changes you wrote in Chapter 04.

## Alternatives

### Keep Raw SQL In App Code

Pros:

- Very direct.
- No generator or ORM setup.

Cons:

- SQL strings can drift from Rust structs.
- Missed queries can compile but behave incorrectly.
- Handlers and domain code learn table details.

### ORM

Pros:

- Gives a higher-level model for common CRUD flows.
- Can reduce handwritten SQL for simple operations.
- Often provides migration and relationship helpers.

Cons:

- SQL behavior can become less visible during review.
- Complex queries may fight the abstraction.
- Engineers still need to understand the generated SQL and database behavior.

### SQLx Checked Queries

Pros:

- Keeps SQL visible.
- Compile-time or offline checks can catch many query/type mismatches.

Cons:

- Queries are still usually embedded in Rust code.
- Does not create a clear DAL boundary by itself.

### SQL-First Generated Code With Cornucopia

Pros:

- SQL stays in named files.
- Rust gets generated typed query functions.
- Reviewers can inspect SQL and Rust boundary separately.
- Fits the Majaz direction better than hiding query behavior behind an ORM.

Cons:

- Requires regeneration when query files change.
- Generated functions still need an application DAL boundary around them.

Why SQL-first now:

- You have lived through manual schema change pain. Now you need to see that raw
  query ownership can drift too.

## Why We Choose This

We choose Cornucopia because SQL remains reviewable as SQL, while Rust gets typed
query access. We still add our own DAL boundary because generated query functions
are persistence primitives, not application policy.

## What Engineers Should Notice

Atlas and Cornucopia solve different problems:

```text
Atlas -> database schema migration history
Cornucopia -> typed Rust access to named SQL queries
DAL -> application boundary that decides how generated queries are used
```

Atlas does not replace Cornucopia. Cornucopia does not replace Atlas. Neither tool
means handlers should own persistence.

## Chapter 07 Tasks

### Task 07.1 - Feel Raw SQL Drift

Input:

- Atlas-managed schema from Chapter 04
- plain SQL in app code

Change:

- add one query shape that returns only published products
- add one insert/update path that touches `description` and `published_at`

Output:

- plain SQL in app code or a temporary query module
- `docs/raw-sql-drift.md` answering:
  - Which SQL strings had to change?
  - Which Rust structs had to change?
  - What could compile while still being wrong?
  - What could leak to production if a query was missed?

Done when:

- the published-products query works
- create/update handles `description` and `published_at`
- you can explain why Atlas did not catch missed Rust query updates

### Task 07.2 - Compare ORM Vs SQL-First Code Generation

Input:

- pain from Task 07.1

Output:

- `docs/dal-alternatives.md`
- comparison table for:
  - Diesel
  - SeaORM
  - SQLx checked queries
  - SQL-first code generation pattern
  - Cornucopia

The table must include:

- pros
- cons
- what it protects against
- what it does not protect against
- why Ahlan uses SQL-first generated code with Cornucopia

Done when:

- you can explain why ORM is valid but not the default path for this project
- you can explain why SQL-first code generation still needs discipline

### Task 07.3 - Introduce Cornucopia

Input:

- Atlas-managed schema
- SQL queries currently owned by app code

Output:

- SQL files under `db/queries/`
- Cornucopia-generated query bindings
- documented regenerate command in `docs/dal.md`
- Make target `make cornucopia-generate`

Expected query files:

See [query-contract.md](query-contract.md).

- `db/queries/products/create_product.sql`
- `db/queries/products/list_products.sql`
- `db/queries/products/list_published_products.sql`
- `db/queries/products/update_product_publication.sql`

Done when:

- generated code compiles
- `make cornucopia-generate` can be rerun
- query file names, query names, parameters, return fields, and list ordering
  match [query-contract.md](query-contract.md)
- you can explain how schema/query changes flow into Rust code

### Task 07.4 - Add The DAL Boundary

Input:

- Cornucopia-generated query bindings

Output:

- `packages/db` or equivalent public-safe package
- product persistence functions that call generated bindings
- handlers/domain orchestration call DAL functions
- tests for create/list/published-list through the DAL

Done when:

- handlers contain no inline SQL
- handlers do not call generated query code directly
- tests prove create, list, and published-list through the DAL
- returned product fields match
  [Chapter 04 product API contract](../04-postgresql-basics/product-api-contract.md)

### Task 07.5 - Document The Boundary

Input:

- working Cornucopia DAL

Output:

- `docs/dal.md`
- explanation of:
  - where schema lives
  - where migrations live
  - where SQL query files live
  - how Atlas and Cornucopia differ
  - when to run Atlas
  - when to run Cornucopia
  - why handlers do not own persistence

Done when:

- a fresh engineer can explain Atlas vs Cornucopia without mixing them up

## Read More

- [Atlas migration directory](https://atlasgo.io/concepts/migration-directory)
- [Diesel guides](https://diesel.rs/guides/)
- [SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/)
- [SQLx `query!` macro](https://docs.rs/sqlx/latest/sqlx/macro.query.html)
- [Cornucopia repository](https://github.com/cornucopia-rs/cornucopia)
- [PostgreSQL DDL documentation](https://www.postgresql.org/docs/current/ddl.html)

## Do Not Add Yet

- GraphQL
- frontend
- worker

## Done When

- You can explain raw SQL drift.
- You can compare ORM, checked queries, and SQL-first generated code.
- Cornucopia-generated bindings are used behind a DAL boundary.
- You can explain what Atlas owns and what Cornucopia owns.

Next: [Specs And Tests](../08-specs-and-tests/README.md)
