# Chapter 12 - Redis Cache

## Why Now

The app now has reads, writes, and multiple processes. It is time to learn cache
behavior with a small concrete use case.

## Learn First

- [Redis data types](https://redis.io/docs/latest/develop/data-types/)
- Basic cache-aside pattern

## Alternatives

### No cache

Pros:

- Simpler and always fresh.

Cons:

- Does not teach performance/freshness tradeoffs.

### In-memory cache

Pros:

- Easy to implement.

Cons:

- Not shared across processes.

### Redis

Pros:

- Shared cache.
- Common production pattern.
- Easy to observe with keys.

Cons:

- Adds another service and failure mode.

Why Redis now:

- You have enough runtime shape to understand why shared cache is different from
  local memory.

## Why We Choose This

We choose Redis because it teaches a shared cache that works across API and
worker processes. A local in-memory cache would hide the process boundary.

## What Engineers Should Notice

Redis is shared runtime infrastructure. It is not the source of truth.

The important change is:

```text
read from database every time -> check shared cache -> fall back to database
```

Caching adds freshness and failure questions. Engineers must know what the cache
key means, when it is invalidated, what happens on a miss, and what happens when
Redis is unavailable.

Correctness must come from PostgreSQL and domain rules. Redis should improve the
read path without making the system wrong when it is empty, stale, or down.

## Chapter 12 Tasks

### Task 12.1 - Add Redis Runtime

Input:

- local mprocs setup

Output:

- Redis service locally
- `make redis-health`
- mprocs entry for Redis

Done when:

- Redis starts with the local stack
- health command proves Redis is reachable

### Task 12.2 - Add Cache Boundary

Input:

- Redis runtime

Output:

- cache module/package
- cache key constants
- helper functions for get/set/delete
- Redis unavailable fallback behavior
- cache primitive behavior for storing, reading, deleting, and safely ignoring
  Redis failures

Expected key format:

See [cache-keys.md](cache-keys.md).

Done when:

- tests prove read/write/delete and fallback behavior
- manual evidence is allowed only when a test is blocked, with the blocker
  documented
- helper functions can store JSON with a TTL, return cache misses safely, and
  continue when Redis is unavailable

### Task 12.3 - Add Cache Observability

Input:

- cache helper

Output:

- cache hit/miss logs
- Redis error logs that do not expose secrets

Done when:

- mentor can see cache hit, miss, and fallback in logs

## Do Not Add Yet

- cache warming worker
- distributed locks
- pub/sub
- full storefront rendering engine
- rendered storefront HTML cache behavior from [cache-contract.md](cache-contract.md)

## Done When

- You can write/read a cache value.
- You can explain the cache key format.
- The app still works when Redis is unavailable.

Next: [Simple Storefront Rendering](../13-simple-storefront-rendering/README.md)
