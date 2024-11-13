# syntax=docker/dockerfile:1

ARG DEPENDENCIES="libssl-dev pkg-config"

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/rust:1.82.0-slim-bookworm@sha256:1111c28d995d06a7863ba6cea3b3dcb87bebe65af8ec5517caaf2c8c26f38010 AS chef
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

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:01d9267d7b0ca810310802fb08cb14579479928ae2e701de9205d829f10aeb94

COPY --from=build /app/target/release/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/mirakurun-client" ]
