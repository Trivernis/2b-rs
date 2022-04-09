ARG QALCULATE_VERSION=4.1.1
ARG BASE_IMAGE=docker.io/alpine:latest

FROM ${BASE_IMAGE} AS build_base
RUN apk update
RUN apk add --no-cache --force-overwrite \
    build-base \
    openssl3-dev \
    libopusenc-dev \
    libpq-dev \
    curl \
    bash
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rm -rf /var/lib/{cache,log}/ /var/cache

FROM build_base AS builder
ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /usr/src
RUN cargo new tobi
WORKDIR /usr/src/tobi
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY bot-coreutils ./bot-coreutils
COPY bot-database ./bot-database
RUN cargo build --release --verbose
RUN mkdir /tmp/tobi
RUN cp target/release/tobi-rs /tmp/tobi/

FROM build_base AS qalculate-builder
ARG QALCULATE_VERSION
RUN mkdir /tmp/qalculate
WORKDIR /tmp/qalculate
RUN apk add --no-cache wget xz ca-certificates
RUN wget https://github.com/Qalculate/qalculate-gtk/releases/download/v${QALCULATE_VERSION}/qalculate-${QALCULATE_VERSION}-x86_64.tar.xz -O qalculate.tar.xz
RUN tar xf qalculate.tar.xz
RUN cp qalculate-${QALCULATE_VERSION}/* /tmp/qalculate

FROM ${BASE_IMAGE} AS runtime-base
RUN apk update
RUN apk add --no-cache --force-overwrite \
    openssl3 \
    libopusenc \
    libpq \
    python3 \
    py3-pip \
    bash
RUN pip3 install youtube-dl
RUN rm -rf /var/lib/{cache,log}/ /var/cache

FROM runtime-base
COPY --from=qalculate-builder /tmp/qalculate/* /usr/bin/
COPY --from=builder /tmp/tobi/tobi-rs .
ENTRYPOINT ["/tobi-rs"]