# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG QSVENCC_VERSION="8.04"

FROM ubuntu:noble-20260113@sha256:cd1dba651b3080c3686ecf4e3c4220f026b521fb76978881737d24f200828b2b
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
