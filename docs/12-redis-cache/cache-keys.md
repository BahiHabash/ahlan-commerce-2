# Chapter 12 Cache Keys

Use named constants for cache keys. Do not inline these strings in handlers.

```text
storefront:product-page:{handle}
```

Meaning:

- `storefront` identifies the read surface.
- `product-page` identifies the cached artifact.
- `{handle}` is the product handle from the storefront route.

Chapter 12 only introduces the key constant and cache helper primitives.
Rendered storefront value shape, TTL, and cache-aside behavior are introduced
when Chapter 13 connects this key to the storefront render path.
