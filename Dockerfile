FROM rust:1.90.0-slim-bookworm as builder

WORKDIR /app

# cache libs
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release

# build app
COPY . .
RUN cargo build --release

# runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/leave-a-note /usr/local/bin/leave-a-note

CMD ["leave-a-note"]