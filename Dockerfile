# syntax=docker/dockerfile:1.0-experimental
FROM rust:latest  AS builder
RUN apt-get update
RUN apt-get install -y build-essential libssl-dev libopus-dev libpq-dev
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

FROM bitnami/minideb:latest
RUN install_packages openssl libopus0 ffmpeg python3 python3-pip libpq5
RUN pip3 install youtube-dl
RUN rm -rf /var/lib/{apt,dpkg,cache,log}/
COPY --from=builder /tmp/tobi/tobi-rs .
ENTRYPOINT ["/tobi-rs"]