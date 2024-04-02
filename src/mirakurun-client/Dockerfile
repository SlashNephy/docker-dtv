# syntax=docker/dockerfile:1

ARG DEPENDENCIES="libssl-dev pkg-config"

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/rust:1.77.1-slim-bookworm@sha256:725959da2e3079f69c33ff2b9b0826d44912e45866cc89239b8700544946aefc AS chef
WORKDIR /app

RUN cargo install cargo-chef

FROM --platform=$TARGETPLATFORM chef AS plan

COPY ./ /app/
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=$TARGETPLATFORM chef AS build
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=plan /app/recipe.json ./
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./ /app/
RUN cargo build --release --workspace --verbose

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:e1d87ce8ed17591e9b10030754f696ac8fbf7a4396372147b568222e14fb0f0a

COPY --from=build /app/target/release/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/mirakurun-client" ]
