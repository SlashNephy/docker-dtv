# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsselect_gcc
ARG TSSELECT_VERSION="r3"

ARG DEPENDENCIES="build-essential"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.47.0@sha256:cf86726f2fea398169b609025f9574c2dee046df634e97b3703127ad9bc2c4d7 AS download
ARG TSSELECT_VERSION

RUN git clone https://github.com/xtne6f/tsselect_gcc -b "${TSSELECT_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:88af55a5d2b47ebd015889a2c000bf013003b095febb788ac3371300e5c8dec8 AS build
WORKDIR /app/src
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ /app/
RUN make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:88af55a5d2b47ebd015889a2c000bf013003b095febb788ac3371300e5c8dec8

COPY --from=build /app/src/tsselect /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsselect" ]
