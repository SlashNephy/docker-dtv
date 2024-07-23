# syntax=docker/dockerfile:1

ARG DEPENDENCIES="libssl-dev pkg-config"

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/rust:1.79.0-slim-bookworm@sha256:cd482d25e50efcc0f9f7b627c0804ec72a3610aa2c70c804d046d535b95f62b3 AS chef
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

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:229473387fee04335e0252359719e6ac1ec80efb08f1e95647458e2a5a58e1ca

COPY --from=build /app/target/release/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/mirakurun-client" ]
