# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/NVEnc
ARG NVENCC_VERSION="7.54"

FROM nvidia/cuda:12.5.0-base-ubuntu22.04@sha256:a89cae4e5e208e62bb188851fa881be73967128e2e5b49b6846b9a2859cb6b82
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
