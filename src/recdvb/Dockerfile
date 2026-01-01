# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=k-pi/recdvb
ARG RECDVB_VERSION="v1.3.3"

ARG DEPENDENCIES="build-essential autoconf automake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.52.0@sha256:22ee857a29be6e98994fe50d9d9730de8ba2535caff6e4d3a1fef48dabaa3713 AS download
ARG RECDVB_VERSION

RUN git clone https://github.com/k-pi/recdvb -b "${RECDVB_VERSION}" /app

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885 AS build
ARG DEPENDENCIES

WORKDIR /app
COPY --from=download /app/ ./
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  chmod +x ./autogen.sh
  ./autogen.sh
  ./configure --prefix=/usr/local
  sed -i -e "s/msgbuf/_msgbuf/" recpt1core.h
  sed -i "1i#include <sys/types.h>" recpt1.h

  make
  make install
EOF

FROM --platform=$TARGETPLATFORM debian:trixie-20251229-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885

COPY --from=build /usr/local/bin/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/recdvb" ]
