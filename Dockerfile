FROM docker.io/rust:1.79.0-bullseye AS builder

WORKDIR /app
COPY . /app
RUN cargo build --release --locked

FROM docker.io/debian:bullseye-20240701-slim
COPY --from=builder /app/target/release/random_reader /usr/bin/random_reader

ENTRYPOINT ["/usr/bin/random_reader"]

