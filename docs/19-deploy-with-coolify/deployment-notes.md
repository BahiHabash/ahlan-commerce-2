# Coolify Deployment Notes

Create services for API, worker, admin, Postgres, and Redis. Configure `DATABASE_URL`, `REDIS_URL`, `API_BIND_ADDR=0.0.0.0:3000`, and `RUN_REFINERY_MIGRATIONS=true` on the API service when using the startup migration hook.

Atlas remains the development migration tool. Production migrations run through the Rust Refinery runner:

```text
make db-migrate-prod
```

Health check:

```text
GET /health
```
