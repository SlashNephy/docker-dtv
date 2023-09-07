# syntax=docker/dockerfile:1
ARG REF="gamma"
ARG RUNTIMES="curl"
ARG DEPENDENCIES="build-essential git sudo ca-certificates python wget xz-utils"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:6662bb5ae28ce4ce754cb4cc4b03deb9b4f24ab052b812b966aa773942e27745 AS downloader

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends git ca-certificates
EOF

FROM --platform=$BUILDPLATFORM downloader AS download
ARG REF

ENV DEBIAN_FRONTEND="noninteractive"
ADD https://api.github.com/repos/Chinachu/Chinachu/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/Chinachu -b "${REF}" --recursive /app

FROM --platform=$TARGETPLATFORM node:19.9.0-buster-slim@sha256:b7575f4bf0b6c5e2eba4fdf3b80e61a2d6ee3ce34b00fef819ffb4430a00962b AS build
ARG RUNTIMES
ARG DEPENDENCIES
ENV DOCKER="YES"

RUN <<EOF
  set -eux

  apt-get update
  apt install -y --no-install-recommends ${RUNTIMES} ${DEPENDENCIES}
EOF

WORKDIR /app
COPY --from=download /app/ ./
RUN <<EOF
  set -eux

  mkdir -p log bin
  chown -R node .
EOF

USER node
RUN <<EOF
  set -eux

  echo 2 | ./chinachu installer
  echo 4 | ./chinachu installer
  echo 5 | ./chinachu installer

  ./chinachu service operator initscript | tee bin/chinachu-operator
  ./chinachu service wui initscript | tee bin/chinachu-wui
EOF

USER root
RUN <<EOF
  set -eux

  mv bin/chinachu-operator bin/chinachu-wui /usr/local/bin/
  rmdir bin
  chmod u+x /usr/local/bin/chinachu-operator /usr/local/bin/chinachu-wui

  # clean up
  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

USER node
COPY ./entrypoint.sh /
CMD ["/entrypoint.sh"]
