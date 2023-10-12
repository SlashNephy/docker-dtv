# syntax=docker/dockerfile:1
ARG REF="master"
ARG RUNTIMES="libpcsclite1"
ARG DEPENDENCIES="build-essential cmake clang pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:fd865da5f887f2fdb7af19c3a33fe9c98cb00a26859df66508cadadc35cb8676 AS downloader

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends git ca-certificates
EOF

FROM --platform=$BUILDPLATFORM downloader AS download
ARG REF

ENV DEBIAN_FRONTEND="noninteractive"
ADD https://api.github.com/repos/kazuki0824/recisdb-rs/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/kazuki0824/recisdb-rs -b "${REF}" --recursive /app

FROM --platform=$TARGETPLATFORM rust:1.73.0-slim-bookworm@sha256:dcade53b45c5f00d952915496317a5db5abfd7d612b010f935bb040f8cc95c37 AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM --platform=$TARGETPLATFORM chef AS plan

COPY --from=download /app/ ./
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=$TARGETPLATFORM chef AS build
ARG RUNTIMES
ARG DEPENDENCIES

COPY --from=plan /app/recipe.json ./
RUN cargo chef cook --release --recipe-path recipe.json

ENV DEBIAN_FRONTEND="noninteractive"
COPY --from=download /app/ ./
RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES} ${DEPENDENCIES}

  cargo build --release --workspace --verbose
EOF

FROM --platform=$TARGETPLATFORM debian:12.1-slim@sha256:24c92a69df28b21676d721fe18c0bf64138bfc69b486746ad935b49cc31b0b91 AS runtime
ARG RUNTIMES
ENV DEBIAN_FRONTEND="noninteractive"

RUN <<EOF
  set -eux

  # install runtimes
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  # clean up
  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=build /app/target/release/recisdb /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/recisdb" ]
