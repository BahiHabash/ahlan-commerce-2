# Chapter 18 Docker And Runtime Contract

Deployment preparation should make every runtime process explicit.

## Required Runtime Processes

- API service
- worker service
- admin frontend build/runtime
- PostgreSQL service
- Redis service

## Required Runtime Notes

Document these in `docs/deployment-prep.md`:

- build command for each build artifact
- start command for API
- start command for worker
- health check command or URL
- migration command
- required env vars
- which service needs each env var
- which service can be restarted independently

## Dockerfile Expectations

- Build from committed source only.
- Do not bake secrets into images.
- Fail startup when required env vars are missing.
- Keep API and worker starts explicit, even if they share one binary.

## Why

Coolify can run the services, but it cannot tell the learner what each service
means. This contract makes deployment a reviewable engineering artifact instead
of a collection of UI clicks.
