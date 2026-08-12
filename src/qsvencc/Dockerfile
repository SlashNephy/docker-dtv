# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG QSVENCC_VERSION="8.04"

FROM ubuntu:noble-20260730.1@sha256:561618e2c15bf2397621dd04f96926663a3b5616c189cf7e38db7e82f5c538ea
ARG QSVENCC_VERSION

ADD https://github.com/rigaya/QSVEnc/releases/download/${QSVENCC_VERSION}/qsvencc_${QSVENCC_VERSION}_Ubuntu24.04_amd64.deb /tmp/package.deb

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  apt-get update
  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists
EOF

ENTRYPOINT [ "/usr/bin/qsvencc" ]
