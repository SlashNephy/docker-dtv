# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=FFmpeg/FFmpeg
ARG RECDVB_VERSION="v1.3.3"

ARG DEPENDENCIES="build-essential autoconf automake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.42.0@sha256:2f4cc0850fe005ffb3e91a48f92bd941acb2888dff910d1f21bb98fdbe3e0876 AS downloader
ARG RECDVB_VERSION

RUN git clone https://github.com/k-pi/recdvb -b "${RECDVB_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:4f5d8d928d9398c9eae7b6512848b9ae6022006a74f816c2fe2cbf8e598ae242 AS builder
ARG DEPENDENCIES

WORKDIR /app
COPY --from=downloader /app/ ./
RUN <<EOF
  apt update
  apt install -y --no-install-recommends ${DEPENDENCIES}

  chmod +x ./autogen.sh
  ./autogen.sh
  ./configure --prefix=/usr/local
  sed -i -e "s/msgbuf/_msgbuf/" recpt1core.h
  sed -i "1i#include <sys/types.h>" recpt1.h

  make
  make install
EOF

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:4f5d8d928d9398c9eae7b6512848b9ae6022006a74f816c2fe2cbf8e598ae242
COPY --from=builder /usr/local/bin/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/recdvb" ]
