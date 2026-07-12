# Chapter 08 Scenario Template

Use this exact structure for every behavior scenario.

```markdown
## PRD-PROD-001 - Valid Product Create

Version: 1 - 2026-06-13

Intent:
Explain the business behavior this scenario protects in one or two sentences.

Given:
- A concrete starting state.
- Required existing data.

When:
- The user or system action happens.

Then:
- Expected API response, database state, emitted event, cache state, or UI state.
- Include exact status codes and important fields when applicable.

Verification:
Automated by: exact_test_function_name

Review:
Status: Pending
Reviewed version: none
Reviewed by: none
Reviewed at: none
```

## Verification Values

`Verification` must be exactly one of:

- `Automated by: exact_test_function_name`
- `Manual: reason and reviewer steps`
- `Future: reason this is out of scope`
- `Blocked: blocker that prevents implementation or automation`

## Version Rule

When `Intent`, `Given`, `When`, or `Then` changes, increment the version number,
set the date to the change date, and reset `Review: Status` to `Pending`.
