# Chapter 02 API Contract

This is the first HTTP contract. Keep it small.
Chapter 04 extends this product shape after the database schema changes. Until
then, implement only the fields shown here.

```text
GET /health
Response 200:
{"status":"ok"}

GET /api/products
Response 200:
{"products":[]}

POST /api/products
Request:
{
  "title": "Coffee Mug",
  "handle": "coffee-mug",
  "price_cents": 2500,
  "inventory_quantity": 12,
  "published": true
}

Response 201:
{
  "product": {
    "id": "uuid",
    "title": "Coffee Mug",
    "handle": "coffee-mug",
    "price_cents": 2500,
    "inventory_quantity": 12,
    "published": true
  }
}
```
