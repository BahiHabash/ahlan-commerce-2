# Chapter 19 - Deploy With Coolify

## Why Now

Local development is working, tested, documented, and reviewed. Now deploy it.

## Learn First

- [Coolify documentation](https://coolify.io/docs)
- GitHub Actions basics

## Alternatives

### Local only

Pros:

- Simple.

Cons:

- Does not teach hosted runtime behavior.

### Generic free hosting

Pros:

- Many options.

Cons:

- Free tiers change often.

### Coolify

Pros:

- Teaches service topology, env vars, logs, and deployment checks.
- Closer to real service deployment.

Cons:

- Requires a Coolify instance.
- Deployment issues may be infrastructure-specific.

Why Coolify now:

- You need to understand the gap between local development and a hosted runtime.

## Why We Choose This

We choose Coolify because it teaches practical service deployment: app services,
databases, Redis, env vars, logs, health checks, and rollback thinking in one
place.

## What Engineers Should Notice

Coolify is where the local process model becomes a hosted service topology.

```text
GitHub repo -> CI -> Coolify services -> logs/health checks -> public URL
```

The deployment is not done when the build succeeds. It is done when the app is
reachable, health checks pass, migrations are understood, and service logs can be
used to debug failures.

## Chapter 19 Tasks

### Task 19.1 - Add CI

Input:

- working local project

Output:

- GitHub Actions workflow
- CI checks based on [ci-contract.md](ci-contract.md)
- CI runs tests, builds, generated docs checks, Cornucopia regeneration checks,
  and either Atlas migration checks or the approved Atlas blocker documented in
  [ci-contract.md](ci-contract.md)

Done when:

- CI passes on GitHub

### Task 19.2 - Deploy Services With Coolify

Input:

- deployment prep docs
- GitHub repo

Output:

- Coolify frontend service
- Coolify API service
- Coolify worker service
- Coolify PostgreSQL service
- Coolify Redis service

Done when:

- public app opens
- API health endpoint works
- worker logs are visible

### Task 19.3 - Write Deployment Notes

Input:

- running Coolify deployment

Output:

- `docs/deployment.md`
- env var names configured in Coolify
- public app URL
- health endpoint URL
- generated docs link
- known rollback or redeploy steps

Done when:

- a mentor can understand the service layout and debug from logs

## Do Not Add Yet

- paid-only hosting assumptions
- manual server changes not documented in the repo
- extra production services
- private Majaz deployment details

## Done When

- Public app opens.
- CI passes.
- Health endpoint works.
- Mentor can understand service layout from docs.

Next: [Final Review](../20-final-review/README.md)
