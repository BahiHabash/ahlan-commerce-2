# Chapter 10 - TanStack React Admin UI

## Why Now

You have a GraphQL API. Now build a small UI that uses it.

## Learn First

- [React Learn](https://react.dev/learn)
- [TanStack Router documentation](https://tanstack.com/router/latest)
- [TanStack Query documentation](https://tanstack.com/query/latest)
- [Apollo Client documentation](https://www.apollographql.com/docs/react/)

## Alternatives

### Server-rendered admin

Pros:

- Less frontend tooling.

Cons:

- Does not teach modern admin app workflows.

### React with TanStack Router and TanStack Query

Pros:

- Teaches component state and data fetching.
- Good fit for interactive admin workflows.
- Router and server-state behavior are explicit.
- Query caching/loading/error states are visible and testable.

Cons:

- Adds TypeScript/frontend tooling.
- Adds Router and Query concepts.

Why TanStack now:

- You have real API behavior to call. The UI is now connected to something
  meaningful.

## Why We Choose This

We choose React with TanStack Router and TanStack Query because it teaches the
admin UI split Majaz expects: URLs/screens are separate from server-state
caching, loading, and invalidation.

## What Engineers Should Notice

TanStack Router, TanStack Query, and the GraphQL client solve different
problems.

The important split is:

```text
TanStack Router -> URL and screen state
TanStack Query -> server state, caching, loading, errors, refetching
GraphQL client -> transport and GraphQL protocol behavior
```

Do not hide server state in React component state when TanStack Query should own
it. Component state is for local UI concerns such as form fields and open/closed
controls. Server state needs cache keys, invalidation, loading states, error
states, and retry/refetch behavior.

## Chapter 10 Tasks

### Task 10.1 - Create The Admin App Shell

Input:

- working GraphQL API

Output:

- React admin app under `apps/admin`
- TanStack Router route for `/products`
- page shell with heading and empty content area

Done when:

- `/products` renders in the browser

### Task 10.2 - Add Product Data Hooks

Input:

- GraphQL query/mutation from chapter 09

Output:

- GraphQL client choice note based on [client-choice.md](client-choice.md)
- deliberately thin GraphQL client wrapper
- TanStack Query hook for product list
- TanStack Query mutation for product create
- cache invalidation after create

Done when:

- hooks expose loading, error, and success states

### Task 10.3 - Build Product List/Create UI

Input:

- product hooks

Output:

- product list table or simple list
- product create form with `title`, `handle`, `description`, `price_cents`,
  `inventory_quantity`, and `published`
- product list GraphQL query reads `title`, `handle`, `description`,
  `priceCents`, `inventoryQuantity`, `published`, `publishedAt`, `createdAt`,
  and `updatedAt`
- product list UI labels may be human-readable, but the data fields come from
  the Chapter 09 camelCase GraphQL schema
- loading, empty, error, and success states

Done when:

- a product can be created from the UI
- the list updates without a full page refresh
- UI fields match the Chapter 09 GraphQL schema and Chapter 04 persisted product
  contract

### Task 10.4 - Add Browser Verification

Input:

- working admin UI

Output:

- e2e test for create product and list update
- `docs/manual-browser-check.md` only if the e2e test is blocked
- blocker note in `docs/manual-browser-check.md` explaining why automation could
  not run and how a mentor repeats the browser check

Done when:

- the browser flow is automated
- if automation is blocked, the blocker and repeatable manual steps are
  documented for mentor approval

## AI Rule

AI may implement small slices only from approved tasks. You must review every
diff.

## Do Not Add Yet

- storefront rendering
- worker dashboard
- Redis cache UI
- design system polish

## Done When

- A product can be created from the UI.
- The list updates.
- You can trace the flow from browser to DB.
- You can explain why TanStack Query owns server state in Ahlan.
- You can explain why Apollo Client is a mentor extension, not the default Ahlan
  implementation.

Next: [Background Worker](../11-background-worker/README.md)
