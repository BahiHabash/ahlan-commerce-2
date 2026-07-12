FROM rust:1.96-slim AS builder

# Set working directory
WORKDIR /usr/src/ahlan-commerce

# Install dependencies needed for compiling certain Rust crates (like OpenSSL and Postgres drivers)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Limit compile jobs to avoid OOM on small VPS builders.
ENV CARGO_BUILD_JOBS=1
ENV CARGO_INCREMENTAL=0
ENV CARGO_PROFILE_RELEASE_DEBUG=0

# Copy all the source code
COPY . .

# Build only the API image binaries.
RUN cargo build --release -p api --bin api --bin refinery-migrate

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binaries from the builder stage
COPY --from=builder /usr/src/ahlan-commerce/target/release/api /usr/local/bin/api
COPY --from=builder /usr/src/ahlan-commerce/target/release/refinery-migrate /usr/local/bin/refinery-migrate

# Expose the API port assuming the standard 3000
EXPOSE 3000

# The startup command is defaulted to the API
CMD ["api"]
