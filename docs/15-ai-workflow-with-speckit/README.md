# Chapter 15 - AI Workflow With Spec Kit

## Why Now

You have built enough manually to understand the system. Now AI can help with
requirements and planning without replacing your judgment.

## Learn First

- [Spec Kit](https://github.com/github/spec-kit)

## Alternatives

### Ad hoc prompting

Pros:

- Fast to start.

Cons:

- Easy to skip requirements and tests.

### Spec Kit workflow

Pros:

- Separates product requirements, technical planning, and tasks.
- Produces reviewable artifacts.

Cons:

- More structured than quick prompting.

Why Spec Kit now:

- You can judge whether the generated requirements and tasks match the system
  because you already built the system manually.

## Why We Choose This

We choose Spec Kit because it makes AI-assisted work reviewable: product intent,
architecture decisions, implementation plan, and executable tasks are separate
artifacts.

## What Engineers Should Notice

Spec Kit changes AI from "write code from a vague idea" into a staged workflow.

The important change is:

```text
discussion -> PRD -> ADR -> plan.md -> tasks.md -> implementation
```

For this book, use the names `PRD` and `ADR` for the first two layers. Keep
Spec Kit native names for `plan.md` and `tasks.md`.

AI can help draft, challenge, and organize the artifacts. The engineer still owns
the decisions, rejects weak assumptions, and checks that every task is
independently verifiable.

## Chapter 15 Tasks

### Task 15.1 - Create Compatibility PRD With Spec Kit

Input:

- working product API, GraphQL API, tests, and docs
- discussion: "Add one compatibility adapter for importing an external product
  payload into native product create behavior"

Output:

- `specs/compatibility-prd.md`
- user story
- acceptance criteria
- out-of-scope list
- edge cases

Done when:

- mentor approves PRD before planning technical work

### Task 15.2 - Create Compatibility ADR

Input:

- approved compatibility PRD

Output:

- `specs/compatibility-adr.md`
- decision: native API plus adapter
- rejected alternatives: public API clone, raw passthrough, native handlers that
  understand external shapes
- consequences

Done when:

- mentor approves ADR before implementation

### Task 15.3 - Create Native Spec Kit Plan And Tasks

Input:

- approved PRD and ADR

Output:

- native Spec Kit `plan.md`
- native Spec Kit `tasks.md`
- tasks are ordered and independently verifiable

Done when:

- each task has expected file/output and verification step

Keep `plan.md` and `tasks.md` names unchanged.

## Do Not Add Yet

- implementation before PRD approval
- ADR before PRD review
- guard-skills before there is an implementation to review
- deployment changes

## Done When

- Mentor approves PRD before ADR.
- Mentor approves ADR before implementation.
- `tasks.md` is agent-executable and independently verifiable.

Next: [Compatibility Adapter](../16-compatibility-adapter/README.md)
