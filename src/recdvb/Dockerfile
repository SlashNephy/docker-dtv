# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=k-pi/recdvb
ARG RECDVB_VERSION="v1.3.3"

ARG DEPENDENCIES="build-essential autoconf automake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.43.2@sha256:a5b0079c401414fdf68d699f9ff2f6b33542668f9f288a339897ab35e1da6a20 AS download
ARG RECDVB_VERSION

RUN git clone https://github.com/k-pi/recdvb -b "${RECDVB_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:462ba540c1ee72896d5d82be7c7fef6ce133ac6b280f22a8f761fbee5a1ef067 AS build
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

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:462ba540c1ee72896d5d82be7c7fef6ce133ac6b280f22a8f761fbee5a1ef067

COPY --from=build /usr/local/bin/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/recdvb" ]
