# syntax=docker/dockerfile:1.0-experimental
FROM rust:alpine AS builder
RUN apk add --no-cache build-base openssl-dev opus-dev postgresql-dev
WORKDIR /usr/src
RUN USER=root cargo new tobi
WORKDIR /usr/src/tobi
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY bot-coreutils ./bot-coreutils
COPY bot-database ./bot-database
COPY bot-serenityutils ./bot-serenityutils
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    cargo build --release
RUN mkdir /tmp/tobi
RUN --mount=type=cache,target=target cp target/release/tobi-rs /tmp/tobi/

FROM alpine
RUN apk add --no-cache build-base openssl opus ffmpeg python3
RUN python3 -m ensurepip
RUN pip3 install youtube-dl
COPY --from=builder /tmp/tobi/tobi-rs .
ENTRYPOINT ["/tobi-rs"]