# Chapter 01 - Rust Project Refresher

## Why Now

Before HTTP, databases, workers, or frontend work, you need a clean Rust project
you understand.

## Learn First

- [The Rust Book: Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [The Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [The Rust Book: Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

## What You Need To Know

- crate vs module
- `Result`
- simple structs/enums
- unit tests
- `cargo run`
- `cargo test`

## Alternatives

Choice:

- Start with a plain Rust binary crate.

Pros:

- Very little setup.
- You focus on Rust basics.
- Errors are easier to understand.

Cons:

- It does not show service boundaries yet.
- You will restructure later.

Why this is right now:

- A fresh Rust base is enough. Workspaces, APIs, DB, and process tools come
  later when there is a reason.

## Why We Choose This

We start with a plain Rust project because the first skill is ownership of domain
code, not framework setup. Majaz uses more structure, but Ahlan earns that
structure gradually.

## What Engineers Should Notice

The first boundary is between product behavior and everything else.

```text
domain types -> domain functions -> tests
```

If this logic is unclear before HTTP exists, adding Axum will only hide the
confusion behind routes and JSON.

## Chapter 01 Tasks

### Task 01.1 - Create The Rust Package

Input:

- `ahlan-commerce` repo from chapter 00

Output:

- Rust package that runs with `cargo run`
- `src/main.rs`
- `src/catalog.rs`

Done when:

- `cargo run` prints a short startup message

### Task 01.2 - Add Product Domain Types

Input:

- empty Rust package

Output:

- `ProductId` type alias or newtype
- `Product` struct with `id`, `title`, `handle`, `price_cents`,
  `inventory_quantity`, and `published`
- `ProductCreate` input struct

Done when:

- the code compiles without HTTP, DB, or serialization requirements

### Task 01.3 - Add In-Memory Product Behavior

Input:

- product domain types

Output:

- `create_product(input: ProductCreate) -> Product`
- `list_products() -> Vec<Product>` or equivalent small in-memory store
- unit tests for create/list

Done when:

- `cargo test` passes
- the tests prove title, handle, price, inventory, and published fields survive
  creation

## Do Not Add Yet

- Axum
- database
- AI-generated code

## Done When

- `cargo test` passes.
- You can explain the domain types.
- You can explain why there is no database yet.

Next: [Axum Basics](../02-axum-basics/README.md)
