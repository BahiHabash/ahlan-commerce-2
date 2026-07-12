# Ahlan Commerce

Ahlan Commerce is a public-safe Rust training commerce service used to practice API, persistence, migrations, workers, caching, documentation, and deployment workflows.

## Quick Start

```sh
cargo test
cargo run
```

The API starts with in-memory storage unless `DATABASE_URL` is set. Atlas is the development migration workflow; Refinery is the production embedded migration runner.
