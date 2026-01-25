# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/NVEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG NVENCC_VERSION="9.10"

FROM nvidia/cuda:13.1.1-base-ubuntu24.04@sha256:e8c8679ccd042249d4c4080a3fab5a6bb52ab6e771addffa2e6e4eafea797bd2
ARG NVENCC_VERSION

ADD https://github.com/rigaya/NVEnc/releases/download/${NVENCC_VERSION}/nvencc_${NVENCC_VERSION}_Ubuntu24.04_amd64.deb /tmp/package.deb

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists
EOF

ENV LD_LIBRARY_PATH="/usr/local/cuda-13/compat:${LD_LIBRARY_PATH}"
ENTRYPOINT [ "/usr/bin/nvencc" ]
