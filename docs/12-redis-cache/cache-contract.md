# Chapter 12 Cache Contract

Use cache-aside for rendered storefront product pages.

## Key

```text
storefront:product-page:{handle}
```

## Value Shape

Store JSON in Redis:

```json
{
  "html": "<!doctype html><html>...</html>",
  "product_id": "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001",
  "product_updated_at": "2026-06-13T10:30:00Z",
  "rendered_at": "2026-06-13T10:31:00Z"
}
```

## TTL

- Set TTL to `300` seconds.
- Invalidation on the product writes that exist in Ahlan is still required.
- TTL is a safety net, not the primary freshness mechanism.

## Read Behavior

1. Try Redis `GET`.
2. If hit and JSON parses, return cached HTML.
3. If miss, Redis error, or invalid JSON, load from PostgreSQL and render.
4. Try Redis `SET` with TTL.
5. If Redis `SET` fails, log the error and still return rendered HTML.

## Write/Invalidation Behavior

After product create and product publication update:

- Delete `storefront:product-page:{handle}`.
- If Redis delete fails, log the error and keep the product write successful.

If a mentor asks the learner to add a product delete endpoint later, that delete
path must also delete `storefront:product-page:{handle}`.

## Required Logs

- cache hit
- cache miss
- cache set failure
- cache delete failure
- Redis unavailable fallback

Each log must include `cache_key` and must not include secrets.
