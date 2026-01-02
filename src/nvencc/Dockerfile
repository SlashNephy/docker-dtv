# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/NVEnc
ARG NVENCC_VERSION="9.08"

FROM alpine:3.23.2@sha256:865b95f46d98cf867a156fe4a135ad3fe50d2056aa3f25ed31662dff6da4eb62 AS downloader
RUN apk add --no-cache github-cli

FROM downloader AS package
ARG NVENCC_VERSION
RUN gh release download "${NVENCC_VERSION}" --repo rigaya/NVEnc -p '*_Ubuntu24.04_amd64.deb' -O /package.deb

FROM nvidia/cuda:13.1.0-base-ubuntu24.04@sha256:ae7ce81a6af05a9a7ad3fe86115cb34db2d024e9c303959679e8889c45aeba73

COPY --from=package /package.deb /tmp/package.deb

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists/*
EOF

ENV LD_LIBRARY_PATH="/usr/local/cuda-13/compat:${LD_LIBRARY_PATH}"
ENTRYPOINT [ "/usr/bin/nvencc" ]
