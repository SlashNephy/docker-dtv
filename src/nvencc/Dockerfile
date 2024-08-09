# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/NVEnc
ARG NVENCC_VERSION="7.61"

FROM nvidia/cuda:12.5.1-base-ubuntu22.04@sha256:47181750209e68cab2e36c3e0e8ab0ab8d76bdabbb418e56e4b0ac7b08b97d34
ARG NVENCC_VERSION

ADD https://github.com/rigaya/NVEnc/releases/download/${NVENCC_VERSION}/nvencc_${NVENCC_VERSION}_Ubuntu22.04_amd64.deb /tmp/package.deb

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists/*
EOF

ENV LD_LIBRARY_PATH="/usr/local/cuda-12.2/compat:${LD_LIBRARY_PATH}"
ENTRYPOINT [ "/usr/bin/nvencc" ]
