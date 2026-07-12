# Chapter 11 - Background Worker

## Why Now

Some work should not happen inside a request. Importing products is a good small
example.

## Learn First

- Basic job queue concepts
- Tokio task basics

## Alternatives

### Synchronous import

Pros:

- Simpler to write.

Cons:

- Request can time out.
- Recovery is unclear.

### Background worker

Pros:

- Work is observable.
- Retries and failures can be modeled.
- Request path stays fast.

Cons:

- Adds process and state complexity.

Why worker now:

- You already have API, DB, and mprocs. A worker is the next process that makes
  multi-process development real.

## Why We Choose This

We choose a background worker because some product work should be accepted by the
API and completed outside the request path. That teaches durable state, retries,
and production debugging.

## What Engineers Should Notice

A worker moves work out of the request path. That changes the correctness model.

The important change is:

```text
request does work now -> request records work -> worker completes work later
```

Once work is asynchronous, engineers must think about job identity, job states,
retry limits, idempotency, logs, and crash recovery. A successful HTTP response
may only mean "job accepted", not "job completed".

Do not hide long-running business work inside a spawned task from a handler
without durable state. If the process dies, that work disappears.

## Chapter 11 Tasks

### Task 11.1 - Add Import Job Schema

Input:

- PostgreSQL migration flow

Output:

- migration or setup SQL based on [import_jobs.sql](import_jobs.sql)
- job state rules based on [import-contract.md](import-contract.md)

Status values:

```text
queued
running
succeeded
failed
```

Done when:

- migration creates `import_jobs`
- status values and retry limits match [import-contract.md](import-contract.md)

### Task 11.2 - Enqueue Jobs From The API

Input:

- import job table

Output:

- `POST /api/import-jobs`
- request and response from [api-contract.md](api-contract.md)
- import input shape from [import-contract.md](import-contract.md)

Done when:

- API creates a queued job row
- invalid import requests use the Chapter 03A error handling rules

### Task 11.3 - Add The Worker Process

Input:

- queued jobs
- product create behavior

Output:

- worker binary/process
- job state transitions
- retry count update
- logs include job ID
- mprocs entry for worker
- fixture import using [fixtures/products-import.json](fixtures/products-import.json)

Done when:

- worker turns a queued job into `succeeded` or `failed`
- failed jobs keep `last_error`
- duplicate product handles follow [import-contract.md](import-contract.md)
- retry behavior follows [import-contract.md](import-contract.md)

## Do Not Add Yet

- queue service
- Redis cache
- scheduled jobs
- multi-worker coordination

## Done When

- API can enqueue a job.
- Worker can complete it.
- Failed jobs are visible.
- You can explain crash recovery.

Next: [Redis Cache](../12-redis-cache/README.md)
