# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsreadex
ARG TSREADEX_VERSION="master-230809"

ARG DEPENDENCIES="build-essential cmake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:30b4e6611de1031f38cf01fbbd8f2f0a503e5cf7b03634efe36ca3f855a9e89e AS download
ARG TSREADEX_VERSION

RUN git clone https://github.com/xtne6f/tsreadex -b "${TSREADEX_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:339dd6901cdbbeff35d6bbf33dc29319a634f373ca664a42299b720649d47cd7 AS build
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ ./
RUN cmake . && make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:339dd6901cdbbeff35d6bbf33dc29319a634f373ca664a42299b720649d47cd7

COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
