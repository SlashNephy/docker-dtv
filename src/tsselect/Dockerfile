# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=xtne6f/tsselect_gcc
ARG TSSELECT_VERSION="r3"

ARG DEPENDENCIES="build-essential"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download
ARG TSSELECT_VERSION

RUN git clone https://github.com/xtne6f/tsselect_gcc -b "${TSSELECT_VERSION}" /app

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885 AS build
WORKDIR /app/src
ARG DEPENDENCIES

RUN apt-get update && apt-get install -y --no-install-recommends ${DEPENDENCIES}

COPY --from=download /app/ /app/
RUN make

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885

COPY --from=build /app/src/tsselect /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/tsselect" ]
