# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsselect_gcc
ARG TSSELECT_VERSION="r3"

ARG DEPENDENCIES="build-essential"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.45.2@sha256:46f0271bedcf85236982a3672357a3c62ed75aee86559f722f158d1657ec0d7d AS download
ARG TSSELECT_VERSION

RUN git clone https://github.com/xtne6f/tsselect_gcc -b "${TSSELECT_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:3e4eca28384ad0218923b9f802c3cbba00cb77f9d745f5d9086c235ff95abcf2 AS build
WORKDIR /app/src
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ /app/
RUN make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:3e4eca28384ad0218923b9f802c3cbba00cb77f9d745f5d9086c235ff95abcf2

COPY --from=build /app/src/tsselect /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsselect" ]
