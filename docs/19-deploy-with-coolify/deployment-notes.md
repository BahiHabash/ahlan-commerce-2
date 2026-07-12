# Coolify Deployment Notes

Create services for API, worker, admin, Postgres, and Redis. Configure `DATABASE_URL`, `REDIS_URL`, and `API_BIND_ADDR=0.0.0.0:3000` on the API service. The API runs embedded Refinery migrations automatically on startup.

Atlas remains the development migration tool. Production migrations run through the Rust Refinery runner:

```text
make db-migrate-prod
```

Health check:

```text
GET /health
```
