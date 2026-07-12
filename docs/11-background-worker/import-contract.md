# Chapter 11 Import Contract

The worker imports products from a small JSON file. Do not add CSV, zip, tar, or
external HTTP imports in the default Ahlan path.

## Input File Shape

```json
{
  "products": [
    {
      "title": "Coffee Mug",
      "handle": "coffee-mug",
      "description": "Ceramic mug for daily coffee.",
      "price_cents": 2500,
      "inventory_quantity": 12,
      "published": true
    }
  ]
}
```

Required fields per product:

- `title`
- `handle`
- `price_cents`
- `inventory_quantity`
- `published`

Optional fields:

- `description`

## Mapping Rules

- Use the same validation rules as
  [Chapter 04 product API contract](../04-postgresql-basics/product-api-contract.md).
- Generate product IDs in Rust as UUIDv7.
- Set timestamps from the application clock.
- If `published` is `true`, set `published_at`.
- If `published` is `false`, set `published_at` to `null`.

## Duplicate Handles

If an imported product handle already exists:

- Do not create a second product.
- Mark the job `failed`.
- Set `last_error` to a safe message that includes the duplicate handle.

## Job State Transitions

Allowed transitions:

```text
queued -> running
running -> succeeded
running -> failed
failed -> queued
```

Retry rules:

- `attempts` starts at `0`.
- Each worker attempt increments `attempts` before importing.
- A job may run at most `3` attempts.
- After `3` failed attempts, keep status `failed`.
- Retrying a failed job means changing status back to `queued` without clearing
  `attempts`.

## Logging

Worker logs must include:

- `job_id`
- `attempt`
- `status`
- `error_code` or safe failure reason when the job fails

Do not log full file contents.
