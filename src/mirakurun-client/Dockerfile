# syntax=docker/dockerfile:1

ARG DEPENDENCIES="libssl-dev pkg-config"

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/rust:1.82.0-slim-bookworm@sha256:9abf10cc84dfad6ace1b0aae3951dc5200f467c593394288c11db1e17bb4d349 AS chef
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

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:88af55a5d2b47ebd015889a2c000bf013003b095febb788ac3371300e5c8dec8

COPY --from=build /app/target/release/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/mirakurun-client" ]
