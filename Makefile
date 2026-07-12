PORT ?= 3000

.PHONY: run test fmt clippy health migrate-dev migrate-prod worker docs-api-check start

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets -- -D warnings

health:
	curl -fsS http://127.0.0.1:$(PORT)/health

migrate-dev:
	atlas migrate apply --env local

migrate-prod:
	RUN_REFINERY_MIGRATIONS=true cargo run

worker:
	cargo run --bin ahlan-worker

docs-api-check:
	cargo test scenario_health_returns_ok

start:
	mprocs "cargo run" "docker compose up postgres redis"
