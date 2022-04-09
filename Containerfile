ARG QALCULATE_VERSION=4.1.1
ARG DEBIAN_RELEASE=bullseye

FROM docker.io/rust:slim-${DEBIAN_RELEASE}  AS builder
RUN apt-get update
RUN apt-get install -y build-essential libssl-dev libopus-dev libpq-dev pkg-config
WORKDIR /usr/src
RUN USER=root cargo new tobi
WORKDIR /usr/src/tobi
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY bot-coreutils ./bot-coreutils
COPY bot-database ./bot-database
RUN cargo build --release
RUN mkdir /tmp/tobi
RUN cp target/release/tobi-rs /tmp/tobi/

FROM docker.io/bitnami/minideb:${DEBIAN_RELEASE} AS runtime-base
RUN apt update
RUN apt install openssl libopus0 ffmpeg python3 python3-pip libpq5 pkg-config -y
RUN pip3 install youtube-dl
RUN rm -rf /var/lib/{apt,dpkg,cache,log}/ /var/cache

FROM docker.io/bitnami/minideb:${DEBIAN_RELEASE} AS qalculate-builder
ARG QALCULATE_VERSION
RUN mkdir /tmp/qalculate
WORKDIR /tmp/qalculate
RUN install_packages ca-certificates wget xz-utils
RUN wget https://github.com/Qalculate/qalculate-gtk/releases/download/v${QALCULATE_VERSION}/qalculate-${QALCULATE_VERSION}-x86_64.tar.xz -O qalculate.tar.xz
RUN tar xf qalculate.tar.xz
RUN cp qalculate-${QALCULATE_VERSION}/* /tmp/qalculate

FROM runtime-base
COPY --from=qalculate-builder /tmp/qalculate/* /usr/bin/
COPY --from=builder /tmp/tobi/tobi-rs .
ENTRYPOINT ["/tobi-rs"]