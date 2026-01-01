# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsreadex
ARG TSREADEX_VERSION="master-230809"

ARG DEPENDENCIES="build-essential cmake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download
ARG TSREADEX_VERSION

RUN git clone https://github.com/xtne6f/tsreadex -b "${TSREADEX_VERSION}" /app

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885 AS build
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ ./
RUN cmake . && make

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885

COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
