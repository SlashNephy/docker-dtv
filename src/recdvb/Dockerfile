# syntax=docker/dockerfile:1

# renovate: datasource=github-tags depName=k-pi/recdvb
ARG RECDVB_VERSION="v1.3.3"

ARG DEPENDENCIES="build-essential autoconf automake"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.42.0@sha256:2f4cc0850fe005ffb3e91a48f92bd941acb2888dff910d1f21bb98fdbe3e0876 AS download
ARG RECDVB_VERSION

RUN git clone https://github.com/k-pi/recdvb -b "${RECDVB_VERSION}" /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:17051768c3a638e86d8f2c4c555194199b562d856e3417fc56216c645843ce3c AS build
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

FROM --platform=$TARGETPLATFORM public.ecr.aws/debian/debian:stable-slim@sha256:17051768c3a638e86d8f2c4c555194199b562d856e3417fc56216c645843ce3c

COPY --from=build /usr/local/bin/ /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/recdvb" ]
