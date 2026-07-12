# Chapter 10 Frontend Client Choice

Ahlan must teach the frontend ownership split without hiding it behind a large
client abstraction too early.

## Required Ahlan Path

Use:

- TanStack Router for URL and screen state
- TanStack Query for server state, loading, errors, retries, and invalidation
- a thin GraphQL request wrapper for the transport call

This keeps the learning goal clear: the candidate can see when server state is
loaded, cached, invalidated, and rendered.

## Optional Mentor Extension

The larger project can also use Apollo Client for richer GraphQL client needs.
Do not use Apollo Client in the default Ahlan implementation. If a mentor wants
stronger frontend parity, add a separate extension after the default task passes:
replace the thin request wrapper with Apollo Client and keep the same product
list/create behavior.

## Tradeoff

| Choice | Pros | Cons | Better for Ahlan |
| --- | --- | --- | --- |
| Thin GraphQL wrapper plus TanStack Query | Clear cache ownership, low cognitive load | Fewer GraphQL client features | Required |
| Apollo Client | Rich GraphQL client, cache, tooling, subscriptions/options | More concepts and a second cache model to explain | Optional mentor extension only |

Do not mix two server-state cache owners in one small exercise unless the mentor
is explicitly teaching why both exist.
