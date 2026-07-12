# Chapter 08 - Specs And Tests

## Why Now

The behavior is no longer trivial. You need reviewable behavior, not only code.

## Learn First

- Basic Given/When/Then examples

## Alternatives

### Tests only

Pros:

- Fast for engineers.

Cons:

- Harder for non-code reviewers to validate behavior.

### Docs only

Pros:

- Easy to read.

Cons:

- Can drift from implementation.

### Specs plus tests

Pros:

- Human-readable behavior.
- Executable verification.
- Clear review trail.

Cons:

- More work to maintain.

Why specs now:

- Product creation/listing has real behavior and persistence. It is worth
  writing down what correctness means.

## Why We Choose This

We choose reviewer-readable specs plus tests because tests alone do not explain
business intent to every reviewer. Scenario IDs give specs, tests, and matrices a
shared language.

## What Engineers Should Notice

Specs are not separate from implementation quality. They define what the tests
claim to protect.

```text
PRD -> scenario IDs -> tests -> implementation behavior
```

Each automated behavior test should map back to a scenario. If a scenario is not
automated, mark it manual, future, or blocked with a reason.

## Chapter 08 Tasks

### Task 08.1 - Write The Product PRD

Input:

- working REST API backed by DAL

Output:

- `specs/product-prd.md`
- user story for product create/list
- acceptance criteria for valid create, duplicate handle, list empty, list with
  products, and invalid input

Done when:

- the PRD explains what success means without reading code

### Task 08.2 - Write Scenario Specs

Input:

- product PRD

Output:

- `specs/product-scenarios.md`
- scenario structure based on [scenario-template.md](scenario-template.md)
- stable scenario IDs:
  - `PRD-PROD-001` valid create
  - `PRD-PROD-002` duplicate handle rejected
  - `PRD-PROD-003` list empty products
  - `PRD-PROD-004` list persisted products
  - `PRD-PROD-005` invalid create input rejected
- each scenario has Version, Intent, Given, When, Then, Verification, and Review

Done when:

- every scenario says `Automated by`, `Manual`, `Future`, or `Blocked`
- every `Review` block starts as `Pending`

### Task 08.3 - Add Tests Mapped To Scenarios

Input:

- scenario specs
- REST API and DAL

Output:

- unit tests for domain validation
- integration tests for API plus DB
- test names referenced from scenario `Verification`
- each automated test name appears in exactly one scenario or names the scenario
  ID it protects

Done when:

- tests pass
- every automated test maps to a scenario ID

## AI Rule

Use AI only to draft clarification questions or explain Given/When/Then. Do not
use Spec Kit yet, and do not let AI implement code yet.

## Do Not Add Yet

- Spec Kit
- guard-skills
- compatibility adapter
- deployment

## Done When

- Each scenario has verification.
- Each scenario follows [scenario-template.md](scenario-template.md).
- Tests map to scenario IDs.
- You can explain what is automated and what is manual/future/blocked.

Next: [GraphQL Slice](../09-graphql-slice/README.md)
