# Chapter 04 Atlas Contract

Use Atlas for database schema migration workflow after you have felt the pain of
manual schema changes.

Minimum files in the learner project:

```text
atlas.hcl
db/schema/products.sql
db/migrations/
```

Minimum commands to document:

```text
atlas migrate diff initial_products --env local
atlas migrate apply --env local
```

Expected behavior:

- `db/schema/products.sql` is the desired product schema.
- `db/migrations/` contains generated migration files.
- applying migrations creates the `products` table in local PostgreSQL.
- Atlas owns schema migration history.
- Atlas does not type-check Rust query usage.

Do not hand-edit production-applied migration files after they are generated.
