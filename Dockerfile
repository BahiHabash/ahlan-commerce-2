FROM rust:1.88-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/ahlan-commerce /usr/local/bin/ahlan-commerce
ENV AHLAN_HOST=0.0.0.0
ENV AHLAN_PORT=3000
EXPOSE 3000
CMD ["ahlan-commerce"]
