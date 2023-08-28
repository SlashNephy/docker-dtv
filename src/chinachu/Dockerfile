ARG REF="gamma"
ARG RUNTIMES="curl"
ARG DEPENDENCIES="build-essential git sudo ca-certificates python wget xz-utils"

FROM --platform=$BUILDPLATFORM debian:stable@sha256:9745edf9e4904fd9ff5fbb2e0c0ba0c28a74e77ca645ed751a0aadff56c439bc AS download
ARG REF

ENV DEBIAN_FRONTEND="noninteractive"
ADD https://api.github.com/repos/Chinachu/Chinachu/git/refs/heads/${REF} /tmp/git.json
RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends git ca-certificates

  git clone https://github.com/Chinachu/Chinachu -b "${REF}" --recursive /app
EOF

FROM --platform=$TARGETPLATFORM node:16.20.2-buster-slim@sha256:728a892464c1f90842b3cefa50b2e4e0ec5211841a35bf8f2f7b9bd686070c64 AS build
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
