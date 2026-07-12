# Runtime Model

Ahlan Commerce runs one Tokio runtime. Axum receives HTTP requests and delegates to handlers. Handlers translate transport input into domain commands, then call the store boundary. Domain code owns product validation, UUIDv7 creation, and application timestamps.

Tower middleware wraps the router for cross-cutting HTTP concerns such as tracing and CORS. The app avoids blocking work inside request handlers; long imports are accepted as jobs and completed by the worker path.
