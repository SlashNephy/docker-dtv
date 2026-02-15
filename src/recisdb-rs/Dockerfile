# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=kazuki0824/recisdb-rs
ARG RECISDB_VERSION="1.2.4"
ARG RUNTIMES="libpcsclite1"
ARG DEPENDENCIES="build-essential cmake clang pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:f1c2b32c4de330955342b3b073ccbdaf772030bd48619f2809231e88578911ba AS download
ARG RECISDB_VERSION

RUN git clone https://github.com/kazuki0824/recisdb-rs -b "${RECISDB_VERSION}" --recursive /app

FROM --platform=$TARGETPLATFORM rust:1.93.1-slim-bookworm@sha256:9eb3d4c46eec8564bbecb6dba21861497989df0c74743d040ea356b3615ab187 AS chef
WORKDIR /app

RUN cargo install cargo-chef

FROM --platform=$TARGETPLATFORM chef AS plan

COPY --from=download /app/ ./
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=$TARGETPLATFORM chef AS build
ARG DEPENDENCIES

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=plan /app/recipe.json ./
RUN cargo chef cook --release --recipe-path recipe.json

COPY --from=download /app/ ./
RUN cargo build --release --workspace --verbose

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885
ARG RUNTIMES

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=build /app/target/release/recisdb /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/recisdb" ]
