# Chapter 00 - Start Here

## Goal

Understand the learning path and the rules of the book.

## Why This Book Is Small First

Choice:

- Start with Ahlan Commerce before entering the full Majaz codebase.

Pros:

- You learn one concept at a time.
- Mistakes are safe and reviewable.
- The final repo can be public on GitHub.

Cons:

- The sandbox is smaller than Majaz.
- Some production details are intentionally missing.

Why it prepares you for Majaz:

- You practice the same engineering habits on a smaller system: clear
  requirements, ownership, tests, docs, caching, workers, deployment, and
  handoff.

## Rules

- Build one repo named `ahlan-commerce`.
- Keep using the same repo through all chapters.
- Do not use private company names in the public repo.
- Do not use Liquid, mayya, or project-specific theme technology.
- Do not introduce a tool before the chapter asks for it.
- Do not use AI to implement code until the book explicitly allows it.

## Chapter 00 Tasks

### Task 00.1 - Create The Public-Safe Repository

Input:

- no Ahlan code yet

Output:

- local repo named `ahlan-commerce`
- `README.md` with one sentence explaining the project
- no private Majaz source copied into the repo

Done when:

- `git status` works inside `ahlan-commerce`
- you can explain why this repo is public-safe

### Task 00.2 - Write The Local Learning Log

Input:

- empty `ahlan-commerce` repo

Output:

- `docs/learning-log.md`
- first entry: what you expect to learn from Ahlan

Done when:

- the log exists and is committed later with the project

## Done When

- You can explain what Ahlan Commerce is.
- You can explain why the book starts smaller than Majaz.
- You have created an empty public-safe repo locally.

Next: [Rust Project Refresher](../01-rust-project-refresher/README.md)
