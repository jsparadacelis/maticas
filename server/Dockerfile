FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/maticas-server /usr/local/bin/maticas-server
EXPOSE 8080
CMD ["maticas-server"]
