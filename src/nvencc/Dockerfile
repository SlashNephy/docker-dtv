# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/NVEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG NVENCC_VERSION="9.10"

FROM nvidia/cuda:13.4.1-base-ubuntu24.04@sha256:3eb2d952d9cff2d7a4e42a1cedc19cfa610a7f72c3b8428811731dcea5a175fd
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
