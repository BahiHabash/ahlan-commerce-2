# Browser Evidence

Automated browser verification is represented by API-level tests in this Rust-only implementation. The React browser path from Chapter 10 is intentionally not implemented because this app was requested as a Rust implementation.

Manual check after `cargo run`:

```text
GET http://127.0.0.1:3000/health
POST http://127.0.0.1:3000/api/products
GET http://127.0.0.1:3000/products/{handle}
```
