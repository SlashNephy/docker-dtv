# syntax=docker/dockerfile:1

ARG REF="gamma"

ARG RUNTIMES="curl"
ARG DEPENDENCIES="build-essential git sudo ca-certificates python3 wget xz-utils"

FROM --platform=$BUILDPLATFORM public.ecr.aws/bitnami/git:2.44.0@sha256:b91c3ac4915cbf6ba813175a0387bca69a70b52e1b57a22e212322036c47da23 AS download
ARG REF

ADD https://api.github.com/repos/Chinachu/Chinachu/git/refs/heads/${REF} /tmp/git.json
RUN git clone https://github.com/Chinachu/Chinachu -b "${REF}" --recursive /app

FROM --platform=$TARGETPLATFORM public.ecr.aws/docker/library/node:22.14.0-bookworm-slim@sha256:6bba748696297138f802735367bc78fea5cfe3b85019c74d2a930bc6c6b2fac4
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
