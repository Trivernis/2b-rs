ARG BASE_IMAGE=docker.io/alpine:edge

FROM ${BASE_IMAGE} AS build_base
RUN apk update
RUN apk add --no-cache --force-overwrite \
    build-base \
    openssl-dev \
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
RUN cargo build --release
RUN mkdir /tmp/tobi
RUN cp target/release/tobi-rs /tmp/tobi/

FROM ${BASE_IMAGE} AS runtime-base
RUN apk update
RUN apk add --no-cache --force-overwrite \
    openssl \
    libopusenc \
    libpq \
    python3 \
    py3-pip \
    qalc \
    ffmpeg \
    bash
RUN pip3 install yt-dlp --break-system-packages
RUN rm -rf /var/lib/{cache,log}/ /var/cache

FROM runtime-base
COPY --from=builder /tmp/tobi/tobi-rs .
ENTRYPOINT ["/tobi-rs"]
