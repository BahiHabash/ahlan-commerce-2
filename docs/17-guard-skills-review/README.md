# Chapter 17 - guard-skills Review

## Why Now

AI-assisted work needs a second-pass quality gate.

## Learn First

- [guard-skills](https://github.com/amElnagdy/guard-skills)

## Alternatives

### Manual review only

Pros:

- Human judgment stays central.

Cons:

- Common AI mistakes can be missed.

### guard-skills plus manual review

Pros:

- Repeatable quality checks.
- Human still decides.

Cons:

- Adds another review step.

Why guard-skills now:

- AI has planned and helped implement a compatibility feature. You need explicit
  checks for code, tests, and docs before handoff.

## Why We Choose This

We choose guard-skills because AI-assisted work needs repeatable checks in
addition to human review. The tool helps find issues; the engineer owns the
final judgment.

## What Engineers Should Notice

guard-skills is a review aid, not an authority.

The important change is:

```text
human-only review -> repeatable guard pass -> human disposition
```

The engineer must record what was accepted, what was fixed, and what was
rejected with reasons. A guard finding is not automatically correct, and a clean
guard pass does not replace understanding the diff.

## Chapter 17 Tasks

### Task 17.1 - Run guard-skills Against The AI-Assisted Change

Input:

- compatibility adapter implementation from Chapter 16
- PRD, ADR, `plan.md`, and `tasks.md`

Output:

- guard-skills review output for code quality
- guard-skills review output for tests
- guard-skills review output for docs

Done when:

- every finding is copied into `docs/guard-review.md`

### Task 17.2 - Disposition Findings

Input:

- guard findings

Output:

- findings
- fixes
- rejected suggestions with reasons

Expected disposition format:

```text
Finding:
Decision: accepted | rejected | needs mentor review
Reason:
Follow-up:
```

Done when:

- accepted findings are fixed
- rejected findings have clear reasons
- mentor-review items are explicitly marked

## Do Not Add Yet

- automatic acceptance of all guard output
- new feature work during review
- deployment before findings are dispositioned

## Done When

- Guard findings are resolved or dispositioned.
- You can explain why each accepted change was made.
- You can explain why each rejected suggestion was rejected.

Next: [Prepare For Deployment](../18-prepare-for-deployment/README.md)
