# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/NVEnc
ARG NVENCC_VERSION="7.82"

FROM nvidia/cuda:13.0.0-base-ubuntu22.04@sha256:c08986c5ea16a41bba86f4ac08b327861c7b7d05dee7fae0648dcf0e964bf6e4
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
