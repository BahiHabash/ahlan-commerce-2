# Chapter 14 - Generated And Written Docs

## Why Now

The project has enough API and workflow surface that people need docs to review
and run it.

## Learn First

- OpenAPI basics
- GraphQL schema documentation basics
- [utoipa](https://docs.rs/utoipa/latest/utoipa/)
- [utoipa-axum](https://docs.rs/utoipa-axum/latest/utoipa_axum/)
- [utoipa-scalar](https://docs.rs/utoipa-scalar/latest/utoipa_scalar/)

## Alternatives

### Written docs only

Pros:

- Easy to explain intent.

Cons:

- API shape can drift.

### Generated API docs only

Pros:

- Close to implementation.

Cons:

- Poor at explaining tradeoffs.

### Both

Pros:

- Machines and humans both get useful docs.

Cons:

- More to maintain.

Why docs now:

- The API is real enough to document. The architecture is real enough to explain.

## Why We Choose This

We choose both generated API docs and written docs because generated docs show
the current contract while written docs explain intent, tradeoffs, setup, and
operations.

## What Engineers Should Notice

Generated docs and written docs answer different questions.

```text
generated docs -> what the API exposes now
written docs -> why the system is shaped this way
```

Generated docs should be reproducible. Written docs should explain decisions and
commands that a new engineer or reviewer needs before changing code.

## Chapter 14 Tasks

### Task 14.1 - Add Generated API Docs

Input:

- REST and GraphQL APIs

Output:

- `docs/api.md`
- generated OpenAPI contract based on [openapi-contract.md](openapi-contract.md)
- `docs/generated/openapi.json`
- local docs route `/docs/scalar`
- exported GraphQL schema at `docs/generated/schema.graphql`
- Make target `make docs-api`

Done when:

- `make docs-api` regenerates `docs/generated/openapi.json` and
  `docs/generated/schema.graphql`
- `/docs/scalar` serves the OpenAPI UI locally

### Task 14.2 - Add Written Project Docs

Input:

- working app and command set

Output:

- `docs/setup.md`
- `docs/architecture.md`
- `docs/commands.md`
- `docs/operations.md`

Done when:

- a mentor can run the project from docs without asking for hidden commands

### Task 14.3 - Add Documentation Checks

Input:

- generated and written docs

Output:

- Make target `make docs-api-check`
- CI step that runs `make docs-api-check`
- `docs/commands.md` entry explaining both `make docs-api` and
  `make docs-api-check`

Done when:

- generated docs are checked in CI and fail when stale

## Do Not Add Yet

- public marketing docs
- full user manual
- private Majaz docs
- undocumented generated files

## Done When

- A mentor can run the project from docs.
- API docs can be regenerated.
- Written docs explain decisions, not just commands.

Next: [AI Workflow With Spec Kit](../15-ai-workflow-with-speckit/README.md)
