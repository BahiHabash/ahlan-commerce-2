# Final Handoff

## Evidence

- Rust workspace: `apps/`, `packages/`
- REST API: `apps/api/src/`
- Error contract: `apps/api/src/error.rs`
- Tracing: `apps/api/src/main.rs`
- ID/time contract: `packages/catalog/src/id.rs`, `packages/catalog/src/clock.rs`
- Atlas dev migrations: `atlas.hcl`, `db/schema/products.sql`, `db/migrations/`
- Refinery prod migrations: `apps/api/src/migrations.rs`, `apps/api/src/bin/refinery-migrate.rs`, `db/refinery_migrations/`
- SQL-first queries: `db/queries/products/`, `db/queries/import_jobs/`
- Cornucopia generated DAL: `packages/db/`
- DAL boundary: `packages/catalog/`, `docs/dal.md`
- GraphQL slice: `apps/api/src/graphql.rs`
- Worker: `apps/worker/`
- Cache/storefront: `packages/cache/`, `apps/api/src/handlers.rs`
- Admin UI: `apps/admin/`
- PRD: `specs/product-prd.md`, `specs/compatibility-prd.md`
- ADR: `docs/compatibility-adr.md`
- Plan/tasks: `plan.md`, `tasks.md`
- CI: `.github/workflows/ci.yml`
- Deployment: `Dockerfile`, `docker-compose.yml`, `docs/operations.md`

## External Links

Public repo URL, public app URL, and CI status link are pending until this local repo is pushed and deployed.
