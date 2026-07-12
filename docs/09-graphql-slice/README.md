# Chapter 09 - GraphQL Slice

## Why Now

You have a REST-like API and persistence. Now learn a typed query/mutation
surface for admin-style workflows.

## Learn First

- [GraphQL Learn](https://graphql.org/learn/)
- [async-graphql book](https://async-graphql.github.io/async-graphql/en/index.html)
- [async-graphql-axum crate docs](https://docs.rs/async-graphql-axum/latest/async_graphql_axum/)

## Alternatives

### REST only

Pros:

- Simple and familiar.

Cons:

- Can become verbose for admin screens that need shaped data.

### GraphQL with `async-graphql`

Pros:

- Strong query shape.
- Good admin UI fit.
- Schema can be documented.
- Rust types and resolvers stay explicit.

Cons:

- Adds schema/resolver concepts.
- Adds a framework-specific integration crate for Axum.

Why GraphQL now:

- The API and DB already exist, so GraphQL can wrap real behavior instead of
  being a toy schema.

## Why We Choose This

We choose a small GraphQL slice because admin workflows often need shaped reads
and mutations. The slice teaches resolver boundaries without replacing the native
domain flow. In Rust, use `async-graphql` with `async-graphql-axum` so the
GraphQL boundary remains inside the Axum/Tokio service model.

## What Engineers Should Notice

GraphQL is not the domain model. It is a transport/query surface over existing
business behavior.

The important change is:

```text
HTTP route -> GraphQL schema -> resolver -> domain flow -> DAL
```

Resolvers should translate request shape into domain calls. They should not
become the place where product rules, persistence rules, or compatibility rules
live.

GraphQL also changes failure and performance thinking. A single request can ask
for nested data, so engineers must watch for accidental per-row queries and keep
batching boundaries explicit.

## Chapter 09 Tasks

### Task 09.1 - Add The GraphQL Schema

Input:

- product domain flow and DAL

Output:

- GraphQL `DateTime` scalar represented as an ISO-8601 UTC timestamp string
- GraphQL `Product` type
- `ProductCreateInput`
- `products` query
- `productCreate` mutation
- `async-graphql` schema root
- Axum route wired through `async-graphql-axum`

Expected schema:

See [schema.graphql](schema.graphql).

Done when:

- schema can be exported or printed
- GraphQL product fields match the persisted product fields from
  [Chapter 04 product API contract](../04-postgresql-basics/product-api-contract.md)
- `publishedAt`, `createdAt`, and `updatedAt` use the `DateTime` scalar from
  [schema.graphql](schema.graphql), not arbitrary strings

### Task 09.2 - Implement Resolvers

Input:

- GraphQL schema
- existing product domain/DAL functions

Output:

- resolver for `products`
- resolver for `productCreate`
- resolvers delegate to domain/DAL code
- GraphQL validation, duplicate handle, not found, dependency unavailable, and
  internal failures expose the Chapter 03A error code in GraphQL error
  extensions

Done when:

- resolver code contains no product persistence SQL

### Task 09.3 - Test GraphQL Behavior

Input:

- working GraphQL resolvers

Output:

- test for product list query
- test for product create mutation
- docs showing one example query and mutation

Done when:

- GraphQL tests pass and behavior matches REST product rules
- tests cover `description`, `publishedAt`, `createdAt`, and `updatedAt`

## Do Not Add Yet

- TanStack React admin UI
- worker
- Redis

## Done When

- You can run a product query.
- You can run a create mutation.
- You can explain resolver vs domain flow.

Next: [TanStack React Admin UI](../10-tanstack-react-admin-ui/README.md)
