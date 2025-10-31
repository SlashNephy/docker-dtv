# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsreadex
ARG TSREADEX_VERSION="master-230809"

ARG DEPENDENCIES="build-essential cmake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.51.2@sha256:05698a08632e71f81ebf9e18f7857777834043fbd49e23b0b52d1bb5cb20fead AS download
ARG TSREADEX_VERSION

RUN git clone https://github.com/xtne6f/tsreadex -b "${TSREADEX_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:01d9267d7b0ca810310802fb08cb14579479928ae2e701de9205d829f10aeb94 AS build
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ ./
RUN cmake . && make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:01d9267d7b0ca810310802fb08cb14579479928ae2e701de9205d829f10aeb94

COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
