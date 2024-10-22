# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/NVEnc
ARG NVENCC_VERSION="7.70"

FROM nvidia/cuda:12.6.2-base-ubuntu22.04@sha256:26c704086505dc1ad1694591566d156296b4cd340a0076396ea8618d4f211bc2
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
