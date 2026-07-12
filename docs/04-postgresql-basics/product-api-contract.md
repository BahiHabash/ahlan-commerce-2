# Chapter 04 Product API Contract

Chapter 04 extends the Chapter 02 product API after `description` and
`published_at` are added to the database schema.

## Create Product

```text
POST /api/products
```

Request:

```json
{
  "title": "Coffee Mug",
  "handle": "coffee-mug",
  "description": "Ceramic mug for daily coffee.",
  "price_cents": 2500,
  "inventory_quantity": 12,
  "published": true
}
```

Rules:

- `title` is required and must not be blank.
- `handle` is required, unique, lowercase, and URL-safe.
- `description` is optional. Missing or `null` is stored as `null`.
- `price_cents` must be greater than or equal to `0`.
- `inventory_quantity` must be greater than or equal to `0`.
- If `published` is `true`, set `published_at` to the application clock time.
- If `published` is `false`, set `published_at` to `null`.
- Generate `id` in Rust as UUIDv7.
- Set `created_at` and `updated_at` from the application clock.

Response `201`:

```json
{
  "product": {
    "id": "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001",
    "title": "Coffee Mug",
    "handle": "coffee-mug",
    "description": "Ceramic mug for daily coffee.",
    "price_cents": 2500,
    "inventory_quantity": 12,
    "published": true,
    "published_at": "2026-06-13T10:30:00Z",
    "created_at": "2026-06-13T10:30:00Z",
    "updated_at": "2026-06-13T10:30:00Z"
  }
}
```

## List Products

```text
GET /api/products
```

Response `200`:

```json
{
  "products": [
    {
      "id": "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001",
      "title": "Coffee Mug",
      "handle": "coffee-mug",
      "description": "Ceramic mug for daily coffee.",
      "price_cents": 2500,
      "inventory_quantity": 12,
      "published": true,
      "published_at": "2026-06-13T10:30:00Z",
      "created_at": "2026-06-13T10:30:00Z",
      "updated_at": "2026-06-13T10:30:00Z"
    }
  ]
}
```

## Errors

Use the Chapter 03A error handling rules and the shared
[error contract](../03-in-memory-product-api/error-contract.md) for validation,
duplicate handle, not found, dependency unavailable, and internal error cases.
