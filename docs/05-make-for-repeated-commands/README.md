# Chapter 05 - Make For Repeated Commands

## Why Now

You now have commands to remember: run API, run tests, start database, run Atlas
migrations. Repeating long commands by hand is error-prone.

## Learn First

- [GNU Make introduction](https://www.gnu.org/software/make/manual/html_node/Introduction.html)

## Alternatives

### Shell scripts

Pros:

- Simple and familiar.

Cons:

- Can become scattered and inconsistent.

### Just

Pros:

- Nice command-runner ergonomics.

Cons:

- Another tool to install.

### Make

Pros:

- Widely available.
- Good enough for common project workflows.
- Easy to call in CI.

Cons:

- Syntax can be surprising.

Why Make now:

- You have already felt the command repetition. Make now solves a real problem
  instead of being introduced as ceremony.

## Why We Choose This

We choose Make because it gives the project a small shared command language that
works locally and in CI. It is not fancy, but it is easy to inspect.

## What Engineers Should Notice

Make is not part of the runtime. It is a project command interface.

The important change is:

```text
remember raw commands -> run named project commands
```

Good Make targets encode workflow decisions without hiding what happens. A new
engineer should still be able to open the `Makefile`, read the command, and
understand which tool is being called.

Use Make for repeated actions such as tests, Atlas migrations, health checks, and
local startup. Do not use it to bury important behavior that belongs in Rust
code, configuration, or deployment docs.

## Chapter 05 Tasks

### Task 05.1 - Capture Repeated Commands

Input:

- API, DB, and Atlas migration commands from chapter 04

Output:

- `Makefile`
- `make run-api`
- `make test`
- `make db-start`
- `make db-migrate`, wrapping `atlas migrate apply --env local`
- `make health`
- keep `make cornucopia-generate` for Chapter 07; do not add it yet

Done when:

- a new engineer can run the common commands without reading shell history

### Task 05.2 - Document The Commands

Input:

- working `Makefile`

Output:

- `docs/commands.md`
- each Make target has one sentence explaining what it does

Done when:

- the docs name the raw command each target wraps

## Do Not Add Yet

- mprocs
- Redis
- worker
- Cornucopia regeneration target before Chapter 07

## Done When

- You no longer need to remember the raw Atlas migration command.
- `make test` works.
- `make health` checks the API.

Next: [mprocs For Multiple Processes](../06-mprocs-for-multiple-processes/README.md)
