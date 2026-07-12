# Product PRD

## Goal

Support a small product catalog with create, list, publish, import, and storefront read flows.

## Requirements

- A product has UUIDv7 identity, title, handle, description, price, inventory, publication state, and timestamps.
- API clients can create products and list products.
- Admin clients can use GraphQL for product list/create/publication flows.
- Storefront readers can view published product pages by handle.
- Import jobs are durable records and can fail independently from enqueue requests.
- Cache is an optimization and not the source of truth.
