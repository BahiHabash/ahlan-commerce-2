# Chapter 06 - mprocs For Multiple Processes

## Why Now

You now run more than one thing: API and Postgres. Soon you will add frontend,
worker, and Redis. Starting each one manually is becoming painful.

## Learn First

- [mprocs GitHub](https://github.com/pvolok/mprocs)

## Alternatives

### Multiple terminal tabs

Pros:

- No new tool.

Cons:

- Logs are scattered.
- Easy to forget a process.

### Docker Compose

Pros:

- Strong service orchestration.

Cons:

- More setup than needed for local process visibility.

### mprocs

Pros:

- Clear multi-process view.
- Grouped logs.
- Lightweight for local development.

Cons:

- Another tool to learn.

Why mprocs now:

- You have felt the process-management problem. mprocs gives one local view
  before the project grows further.

## Why We Choose This

We choose mprocs for local development because it keeps several process logs in
one place without pretending to be the production orchestrator.

## What Engineers Should Notice

mprocs does not orchestrate production. It gives local development one visible
process board.

The important change is:

```text
separate terminal tabs -> one local process view
```

That matters because modern service work is rarely one process. Engineers need
to see API logs, database status, frontend logs, workers, and Redis together so
they can debug request flow across process boundaries.

Keep mprocs as a development workflow tool. Production service topology belongs
in deployment configuration and deployment docs.

## Chapter 06 Tasks

### Task 06.1 - Add Local Process Board

Input:

- `make run-api`
- `make db-start` or DB health command

Output:

- mprocs config for API and PostgreSQL
- `make start`
- `make stop` if needed

Done when:

- `make start` opens one view with API and DB logs

### Task 06.2 - Add Local Debug Notes

Input:

- working mprocs setup

Output:

- `docs/local-runtime.md`
- notes showing where to find API logs and DB logs

Done when:

- you can explain why mprocs is local workflow, not production deployment

## Do Not Add Yet

- frontend
- worker
- Redis

## Done When

- `make start` opens the multi-process view.
- You can find API logs and DB logs.
- You can stop the local stack cleanly.

Next: [SQL-First DAL](../07-sql-first-dal/README.md)
