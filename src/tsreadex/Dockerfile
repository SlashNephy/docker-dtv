# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsreadex
ARG TSREADEX_VERSION="master-230809"

ARG DEPENDENCIES="build-essential cmake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download
ARG TSREADEX_VERSION

RUN git clone https://github.com/xtne6f/tsreadex -b "${TSREADEX_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:13.2-slim@sha256:999c5a6519ee511a57c0bd87010d592f15af840a3399bb1cce33fe16d8fcbe37 AS build
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ ./
RUN cmake . && make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:13.2-slim@sha256:999c5a6519ee511a57c0bd87010d592f15af840a3399bb1cce33fe16d8fcbe37

COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
