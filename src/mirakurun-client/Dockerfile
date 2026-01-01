# syntax=docker/dockerfile:1

ARG DEPENDENCIES="libssl-dev pkg-config"

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/rust:1.92.0-slim-bookworm@sha256:a8ce22642819f5f54d37e025f7020b51dcbac39e5d0ea907a63eda8b03458de5 AS chef
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

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885

COPY --from=build /app/target/release/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/mirakurun-client" ]
