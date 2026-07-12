# Operations

## Environment

See `docs/18-prepare-for-deployment/env.example`.

`DATABASE_URL` enables PostgreSQL. Without it, the API uses in-memory storage for local exercises and tests.

`RUN_REFINERY_MIGRATIONS=true` runs embedded production migrations before the API starts.

## Start Commands

API:

```sh
cargo run
```

Production migration plus API:

```sh
RUN_REFINERY_MIGRATIONS=true cargo run
```

Dev migrations:

```sh
atlas migrate apply --env local
```
