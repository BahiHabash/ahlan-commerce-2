# Chapter 16 - Compatibility Adapter

## Why Now

You understand native API behavior, docs, tests, and AI workflow. Now add a
small adapter without turning the native API into a clone of another platform.

## Learn First

- Basic adapter pattern
- Fixture-based tests
- ADR format

## Alternatives

### Copy the external API shape directly

Pros:

- Fast for one example.

Cons:

- Native model becomes unclear.
- Harder to evolve independently.

### Native API plus adapter

Pros:

- Native behavior stays platform-owned.
- Compatibility behavior is isolated.
- Fixture tests are focused.

Cons:

- Requires mapping code.

Why this now:

- The native product API is stable enough to adapt. Before now, an adapter would
  hide unclear core behavior.

## Why We Choose This

We choose native API plus adapter because compatibility should help external
shapes enter the system without making the native model a clone of another
platform.

## What Engineers Should Notice

A compatibility adapter is a boundary. It should not redefine the native product
model.

The important change is:

```text
external shape -> adapter mapping -> native command -> native behavior
```

Native tests should prove native behavior without mentioning the external shape.
Adapter tests should prove the mapping from external fixtures into native
commands. This keeps compatibility useful without making the public API a clone
of another platform.

## Chapter 16 Tasks

### Task 16.1 - Add External Product Fixture

Input:

- approved PRD, ADR, `plan.md`, and `tasks.md`

Output:

- fixture based on [external-product.json](external-product.json)

Expected fixture shape:

See [external-product.json](external-product.json).

Done when:

- fixture is committed and documented as external input

### Task 16.2 - Map External Shape To Native Command

Input:

- external product fixture
- native `ProductCreate` behavior

Output:

- adapter function that maps:
  - `name` -> `title`
  - `slug` -> `handle`
  - `price` -> `price_cents`
  - `stock` -> `inventory_quantity`
  - `is_visible` -> `published`

Done when:

- adapter returns native input without calling HTTP handlers directly

### Task 16.3 - Test Native And Adapter Boundaries

Input:

- adapter function

Output:

- native tests that do not mention external fixture keys
- adapter tests that use `fixtures/external-product.json`

Done when:

- native tests prove native behavior
- adapter tests prove external mapping
- no public API is presented as a clone of another platform

## Do Not Add Yet

- public API clone of another platform
- passthrough GraphQL proxy
- compatibility behavior inside native handlers
- broad adapter coverage beyond one product fixture

## Done When

- You can explain native behavior separately from compatibility behavior.
- Native tests pass without compatibility fixtures.
- Adapter tests prove the external shape maps correctly.

Next: [guard-skills Review](../17-guard-skills-review/README.md)
