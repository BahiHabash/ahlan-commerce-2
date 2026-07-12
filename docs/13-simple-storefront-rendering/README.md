# Chapter 13 - Simple Storefront Rendering

## Why Now

You have products and Redis. Now create a tiny storefront read path and cache
the rendered result.

## Learn First

- Basic HTML response in Axum
- Cache-aside rendering flow

## Alternatives

### Client-rendered storefront

Pros:

- Frontend-only rendering.

Cons:

- Does not teach server render/cache behavior.

### Simple server-rendered HTML

Pros:

- Easy to inspect.
- Good for learning request context and cache behavior.

Cons:

- Much simpler than a real theme engine.

Why simple HTML now:

- You need the storefront concept, not project-specific theme technology. Do not
  use Liquid, mayya, or private theme engine code.

## Why We Choose This

We choose simple server-rendered HTML because the onboarding goal is the
storefront read path and cache behavior, not Majaz's private rendering engine.

## What Engineers Should Notice

Storefront rendering is a read path with user-visible correctness. It is not only
string building.

The important change is:

```text
product data -> render context -> HTML response -> cached rendered result
```

The context builder should own what data the page needs. The renderer should own
how that data becomes HTML. Cache code should own freshness and invalidation.

Keep those boundaries separate even in the simple version. Do not introduce
Liquid, mayya, themes, sections, checkout, or tenant behavior in Ahlan.

## Chapter 13 Tasks

### Task 13.1 - Add Storefront Route

Input:

- product read behavior

Output:

- `GET /products/{handle}`
- 200 HTML response for published product
- 404 for missing or unpublished product
- exact route contract in [storefront-contract.md](storefront-contract.md)

Done when:

- browser opens a product page by handle

### Task 13.2 - Split Context, Renderer, And Cache

Input:

- storefront route

Output:

- render context builder
- simple HTML renderer
- Redis cache read/write around the render path
- rendered HTML cache behavior based on
  [../12-redis-cache/cache-contract.md](../12-redis-cache/cache-contract.md)

Done when:

- you can explain which code loads data, which code renders HTML, and which code
  owns freshness
- cache value shape, TTL, and Redis-down behavior match
  [../12-redis-cache/cache-contract.md](../12-redis-cache/cache-contract.md)

### Task 13.3 - Invalidate On Product Change

Input:

- cached product page
- product create path
- product publication update path from Chapter 07

Output:

- cache invalidation for `storefront:product-page:{handle}`
- tests for hit, miss, invalidation, and Redis fallback
- manual evidence only if a test is blocked, with the blocker documented

Done when:

- creating or changing publication for a product does not serve stale HTML

## Do Not Add Yet

- Liquid
- mayya
- private theme engine code
- checkout
- multi-store tenancy

## Done When

- Product page renders HTML.
- Cache hit/miss is visible.
- Product create and publication update invalidate cache.
- Redis outage does not break correctness.

Next: [Generated And Written Docs](../14-generated-and-written-docs/README.md)
