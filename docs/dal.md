# DAL Boundary

Atlas owns local development schema history and diffs from `db/schema/products.sql` into `db/migrations/`.

Refinery owns production migration execution. The Rust binary embeds `db/refinery_migrations/` and applies those migrations when `RUN_REFINERY_MIGRATIONS=true`.

Cornucopia owns the SQL-first query contract. Named SQL files live under `db/queries/products/`; generated bindings are intentionally kept behind the DAL boundary. Handlers call `ProductStore`, not generated query functions or inline SQL.
