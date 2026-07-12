# Scenarios

## AC-PRODUCT-001 Create Product

Given a valid create command, when the API receives it, then the product is stored with a UUIDv7 ID and application timestamps.

Verified by `scenario_product_create_sets_id_and_time`.

## AC-PRODUCT-002 Validation Error

Given an invalid create command, when the API receives it, then the response uses the public error envelope and does not leak internal causes.

Verified by `scenario_validation_error_uses_public_contract`.

## AC-HEALTH-001 Health

Given the API is running, when `/health` is requested, then the response is `200 OK`.

Verified by `scenario_health_returns_ok`.
