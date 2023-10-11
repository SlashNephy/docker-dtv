# syntax=docker/dockerfile:1
ARG REF="gamma"
ARG RUNTIMES="curl"
ARG DEPENDENCIES="build-essential git sudo ca-certificates python wget xz-utils"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:fd865da5f887f2fdb7af19c3a33fe9c98cb00a26859df66508cadadc35cb8676 AS downloader

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

FROM --platform=$TARGETPLATFORM node:16.20.2-buster-slim@sha256:3ebf2875c188d22939c6ab080cfb1a4a6248cc86bae600ea8e2326aa03acdb8f AS build
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
