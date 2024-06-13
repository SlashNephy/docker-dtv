# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsreadex
ARG TSREADEX_VERSION="master-230809"

ARG DEPENDENCIES="build-essential cmake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:b91c3ac4915cbf6ba813175a0387bca69a70b52e1b57a22e212322036c47da23 AS download
ARG TSREADEX_VERSION

RUN git clone https://github.com/xtne6f/tsreadex -b "${TSREADEX_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:1620ae46786ee677bf7e9fe9dab0c22cac1b0462fef0a06aaa0df0b4b40a0df2 AS build
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ ./
RUN cmake . && make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:1620ae46786ee677bf7e9fe9dab0c22cac1b0462fef0a06aaa0df0b4b40a0df2

COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
