ARG REF="master"
ARG RUNTIMES="libpcsclite1"
ARG DEPENDENCIES="build-essential cmake clang pkg-config libpcsclite-dev"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:9745edf9e4904fd9ff5fbb2e0c0ba0c28a74e77ca645ed751a0aadff56c439bc AS download
ARG REF

ENV DEBIAN_FRONTEND="noninteractive"
ADD https://api.github.com/repos/kazuki0824/recisdb-rs/git/refs/heads/${REF} /tmp/git.json
RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends git ca-certificates

  git clone https://github.com/kazuki0824/recisdb-rs -b "${REF}" --recursive /app
EOF

FROM --platform=$TARGETPLATFORM rust:1.72.0-slim-bookworm@sha256:f7a362d746c5ac96d98afd299894d710fef87f45946e9b37d7cd7710499858ba AS chef

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

FROM --platform=$TARGETPLATFORM debian:12.1-slim@sha256:741bae561f5c2261f4cdd535e4fd4c248dec0aafc1b9a1410b3d67ad24571340 AS runtime
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
