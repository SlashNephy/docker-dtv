# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsreadex
ARG TSREADEX_VERSION="master-230809"

ARG DEPENDENCIES="build-essential cmake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.42.0@sha256:2f4cc0850fe005ffb3e91a48f92bd941acb2888dff910d1f21bb98fdbe3e0876 AS download
ARG TSREADEX_VERSION

RUN git clone https://github.com/xtne6f/tsreadex -b "${TSREADEX_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:33b9675f63500341c7119739d39d0f6cd776cdf34499494e3e793da2f45f1bfe AS build
WORKDIR /app
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ ./
RUN cmake . && make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:33b9675f63500341c7119739d39d0f6cd776cdf34499494e3e793da2f45f1bfe

COPY --from=build /app/tsreadex /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsreadex" ]
