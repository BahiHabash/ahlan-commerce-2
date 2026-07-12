# Coolify Deployment Notes

Create services for API, Postgres, and Redis. Configure `DATABASE_URL`, `REDIS_URL`, `AHLAN_HOST=0.0.0.0`, `AHLAN_PORT=3000`, and `RUN_REFINERY_MIGRATIONS=true` on the API service.

Health check:

```text
GET /health
```
