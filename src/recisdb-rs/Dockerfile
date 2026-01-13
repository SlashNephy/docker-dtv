# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=kazuki0824/recisdb-rs
ARG RECISDB_VERSION="1.2.3"
ARG RUNTIMES="libpcsclite1"
ARG DEPENDENCIES="build-essential cmake clang pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM alpine/git:2.52.0@sha256:b4ba5e95ee803a9f98fe9f4306e0105f679d2386dedaf207ffe62dd20fdd3f1c AS download
ARG RECISDB_VERSION

RUN git clone https://github.com/kazuki0824/recisdb-rs -b "${RECISDB_VERSION}" --recursive /app

FROM --platform=$TARGETPLATFORM rust:1.92.0-slim-bookworm@sha256:2c64ec2a1aea989c8fc044a98493ee5424884bb8ec5f95dca75e9ff77e8391fa AS chef
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
