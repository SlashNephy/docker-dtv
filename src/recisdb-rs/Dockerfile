# syntax=docker/dockerfile:1
ARG REF="master"
ARG RUNTIMES="libpcsclite1"
ARG DEPENDENCIES="build-essential cmake clang pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:d86f367afb53d022acc4377741e7334bc20add161bb10234272b91b459b4b7d8 AS download
ARG REF

ADD https://api.github.com/repos/kazuki0824/recisdb-rs/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/kazuki0824/recisdb-rs -b "${REF}" --recursive /app

FROM --platform=$TARGETPLATFORM rust:1.92.0-slim-bookworm@sha256:a8ce22642819f5f54d37e025f7020b51dcbac39e5d0ea907a63eda8b03458de5 AS chef
WORKDIR /app

RUN cargo install cargo-chef

FROM --platform=$TARGETPLATFORM chef AS plan

COPY --from=download /app/ ./
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=$TARGETPLATFORM chef AS build
ARG RUNTIMES
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${RUNTIMES} ${DEPENDENCIES}

COPY --from=plan /app/recipe.json ./
RUN cargo chef cook --release --recipe-path recipe.json

COPY --from=download /app/ ./
RUN cargo build --release --workspace --verbose

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885
ARG RUNTIMES

RUN <<EOF
  set -eux

  # install runtimes
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=build /app/target/release/recisdb /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/recisdb" ]
