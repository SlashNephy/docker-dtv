# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsselect_gcc
ARG TSSELECT_VERSION="r3"

ARG DEPENDENCIES="build-essential"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download
ARG TSSELECT_VERSION

RUN git clone https://github.com/xtne6f/tsselect_gcc -b "${TSSELECT_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:13.2-slim@sha256:999c5a6519ee511a57c0bd87010d592f15af840a3399bb1cce33fe16d8fcbe37 AS build
WORKDIR /app/src
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ /app/
RUN make

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:13.2-slim@sha256:999c5a6519ee511a57c0bd87010d592f15af840a3399bb1cce33fe16d8fcbe37

COPY --from=build /app/src/tsselect /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsselect" ]
