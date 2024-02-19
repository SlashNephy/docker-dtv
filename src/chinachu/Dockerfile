# syntax=docker/dockerfile:1

ARG REF="gamma"

ARG RUNTIMES="curl"
ARG DEPENDENCIES="build-essential git sudo ca-certificates python3 wget xz-utils"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.43.2@sha256:678d6f8f4c1fb9b6bffa683edc4cccb884a37f55b3a4619b00e34874c2c5a442 AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/Chinachu/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/Chinachu -b "${REF}" --recursive /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/node:16.20.2-bookworm-slim@sha256:89061680359f5d9103d24f0066e2c96a8018c24d5c4fe8eb8d59d16dd4ab64ba
WORKDIR /app
ARG RUNTIMES
ARG DEPENDENCIES
ENV DOCKER="YES"

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES} ${DEPENDENCIES}
  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=download /app/ ./
RUN mkdir -p log bin && chown -R node .

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
EOF

COPY ./entrypoint.sh /

USER node
CMD ["/entrypoint.sh"]
