# ADR: Compatibility Adapter

## Decision

Use a small adapter module that deserializes the external product shape and returns native `CreateProduct` commands.

## Alternatives

- Expose external fields directly in native handlers.
- Store raw external payloads as products.

## Consequences

Native tests remain focused on native behavior. Adapter tests cover external field mapping separately.
