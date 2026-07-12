# Chapter 19 CI Contract

CI is part of the handoff evidence. It should prove the public repo can be
validated without relying on a mentor's machine.

## Required Checks

- Rust formatting check: `cargo fmt --check`.
- Rust lint/check step: `cargo clippy --workspace --all-targets`.
- Rust unit tests: `cargo test`.
- Rust integration tests: project-specific integration test command documented
  in `docs/commands.md`.
- Frontend typecheck or build: project-specific frontend command documented in
  `docs/commands.md`.
- Generated GraphQL/OpenAPI docs check: `make docs-api-check`.
- Atlas migration check: exact Atlas check/apply command documented in
  `docs/commands.md`.
- Cornucopia regeneration check: `make cornucopia-generate` followed by a git
  diff/staleness check.

## Acceptable Manual Fallback

Do not use manual fallback for generated docs or Cornucopia regeneration.

Manual fallback is allowed only for an Atlas check that is blocked by a missing
database service in GitHub Actions. If that happens, document the blocker in
`docs/deployment.md`, include the exact local Atlas command a mentor should run,
and keep the rest of CI required.

## Done Check

The GitHub Actions page shows a passing workflow, and the final handoff links to
that run.
