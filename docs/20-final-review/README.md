# Chapter 20 - Final Review

## Why Now

This is where you prove readiness for guided Majaz work.

## Required Evidence

- public GitHub repo
- public app URL
- CI status
- PRD
- ADR
- `plan.md`
- `tasks.md`
- scenario specs
- unit tests
- integration tests
- end-to-end test evidence
- documented manual browser evidence only when linked to an automation blocker
- generated API docs
- written setup docs
- Chapter 03A error-handling implementation, tests, and
  `docs/logs/root-cause-error.log`
- Chapter 03B tracing implementation, `docs/logs/success-request.log`, and
  `docs/logs/failed-request.log`
- ID/time contract
- architecture notes
- compatibility adapter ADR and fixture tests
- deployment preparation notes
- Docker/runtime contract
- deployment notes
- CI contract and CI status link
- guard-skills findings and dispositions
- final handoff summary

## Chapter 20 Tasks

### Task 20.1 - Build The Handoff Packet

Input:

- completed Ahlan project

Output:

- `docs/final-handoff.md`
- evidence list based on [handoff-checklist.md](handoff-checklist.md)
- public GitHub repo URL
- public app URL
- CI status link
- links to PRD, ADR, `plan.md`, `tasks.md`, tests, docs, deployment notes, and
  guard review
- link to the end-to-end test; if blocked, link to the blocker note and
  repeatable manual browser evidence
- link to Chapter 03A error-handling implementation, tests, and
  `docs/logs/root-cause-error.log`
- links to Chapter 03B tracing implementation, `docs/logs/success-request.log`,
  and `docs/logs/failed-request.log`
- links to ID/time, Docker/runtime, and CI contracts

Done when:

- a mentor can review all evidence from one file

### Task 20.2 - Run The Final Explanation

Input:

- final handoff packet

Output:

- recorded or written walkthrough answering the mentor questions below

Done when:

- you can explain the system without reading code line by line

## Mentor Questions

- Can you explain the system from request to database?
- Can you explain domain ownership?
- Can you explain route/config constants?
- Can you explain the public error envelope and private root-cause context?
- Can you explain what request tracing fields are logged and why?
- Can you explain where UUIDv7 IDs and timestamps are created?
- Can you explain the Cornucopia-generated DAL?
- Can you explain Redis cache keys and invalidation?
- Can you explain worker recovery?
- Can you explain generated OpenAPI docs vs written docs?
- Can you explain how the deployed API and worker start?
- Can you explain what CI proves before Coolify deploys?
- Can you explain why PRD and ADR exist?
- Can you explain every AI-assisted diff?
- Can you debug the Coolify deployment from logs and docs?

## Completion Standard

You are ready when you can explain this full path:

```text
idea -> PRD -> ADR -> plan.md -> tasks.md -> implementation
-> tests -> docs -> guards -> CI -> Coolify -> handoff
```

Next: [Back To Index](../README.md)
