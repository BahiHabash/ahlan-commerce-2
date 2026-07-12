# Chapter 11 Import Job API Contract

```text
POST /api/import-jobs
Request:
{"input_path":"fixtures/products.json"}

Response 202:
{"job":{"id":"uuid","status":"queued"}}
```

Rules:

- `input_path` must point to a JSON file that matches
  [import-contract.md](import-contract.md).
- The API only validates that the request is structurally valid and records a
  queued job.
- The worker validates and imports file content.
- Invalid request bodies use Chapter 03A `validation_failed`.
- Missing input files make the job `failed`; they do not make the enqueue
  request fail after the job is accepted.

Allowed status values:

```text
queued
running
succeeded
failed
```
