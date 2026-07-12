# Chapter 18 - Prepare For Deployment

## Why Now

Before Coolify, the project needs production-shaped build and runtime settings.

## Learn First

- Dockerfile basics
- Environment variables and secrets
- Production build commands

## Alternatives

### Deploy directly from local assumptions

Pros:

- Faster to try.

Cons:

- Fails unpredictably in hosted runtime.

### Prepare build/runtime boundaries first

Pros:

- Deployment errors are easier to diagnose.
- Env vars and health checks are explicit.

Cons:

- Adds a step before seeing the app online.

Why this now:

- The local app, tests, docs, and operations are already understandable. This is
  the right time to make the runtime portable.

## Why We Choose This

We prepare deployment before using Coolify because hosted runtime failures are
easier to diagnose when build commands, env vars, health checks, and migration
commands are already explicit.

## What Engineers Should Notice

Deployment preparation turns local assumptions into runtime contracts.

```text
local commands -> build artifact -> env vars -> health check -> migration command
```

Every required setting should be named. Missing secrets should fail clearly at
startup, not become hidden defaults.

## Chapter 18 Tasks

### Task 18.1 - Define Runtime Configuration

Input:

- working API, admin UI, worker, Postgres, and Redis

Output:

- `.env.example` based on [env.example](env.example)
- required env var list in `docs/deployment-prep.md`
- no production secret values committed

Required env vars:

See [env.example](env.example).

Done when:

- missing required vars fail at startup with clear errors

### Task 18.2 - Define Build And Start Commands

Input:

- local commands and Make targets

Output:

- API build command
- admin build command
- API start command
- worker start command
- migration command
- health check command

Done when:

- commands are documented in `docs/deployment-prep.md`

### Task 18.3 - Add Deployment Artifacts

Input:

- build/start commands

Output:

- Dockerfile or documented build config for API
- runtime contract based on [docker-contract.md](docker-contract.md)
- documented frontend build config
- documented worker start command

Done when:

- a mentor can identify how each service starts
- a mentor can identify which process owns health checks, migrations, and
  runtime env vars

## Do Not Add Yet

- Coolify services before build/runtime docs exist
- production secrets in git
- undocumented env vars
- manual-only deployment steps

## Done When

- You can build the API for deployment.
- You can build the frontend for deployment.
- You can list every required env var.
- You can explain which command runs migrations.

Next: [Deploy With Coolify](../19-deploy-with-coolify/README.md)
