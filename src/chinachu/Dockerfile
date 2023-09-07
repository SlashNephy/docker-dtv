# syntax=docker/dockerfile:1
ARG REF="gamma"
ARG RUNTIMES="curl"
ARG DEPENDENCIES="build-essential git sudo ca-certificates python wget xz-utils"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:9745edf9e4904fd9ff5fbb2e0c0ba0c28a74e77ca645ed751a0aadff56c439bc AS downloader

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

FROM --platform=$TARGETPLATFORM node:16.20.2-buster-slim@sha256:f576647ee9f745dd21ce7774130a8ea4471afd3dc151b76bfc3800a96f5802f6 AS build
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
