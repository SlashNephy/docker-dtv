# syntax=docker/dockerfile:1
ARG REF="master"
ARG RUNTIMES="libpcsclite1"
ARG DEPENDENCIES="build-essential cmake clang pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.46.0@sha256:5b5561eb89121edc70dcae3caa5ccd71934a64b606a2a847e40cec390d38ae04 AS download
ARG REF

ADD https://api.github.com/repos/kazuki0824/recisdb-rs/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/kazuki0824/recisdb-rs -b "${REF}" --recursive /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/rust:1.80.1-slim-bookworm@sha256:894a3547d9e5b0eaa8463f7e6663201c3c48d7f4e719d933521e1fc075868478 AS chef
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

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:339dd6901cdbbeff35d6bbf33dc29319a634f373ca664a42299b720649d47cd7
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
