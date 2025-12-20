FROM rust:1.91.1-alpine AS builder

WORKDIR /streaming-api

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add g++ libc-dev cmake make

COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo fetch
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /streaming-api/target/x86_64-unknown-linux-musl/release/streaming_api /usr/local/bin/streaming_api

EXPOSE 8181
CMD streaming_api --kafka-brokers "$KAFKA_ADDRESS" --ws-address "$WS_ADDRESS"
