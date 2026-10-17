# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG QSVENCC_VERSION="8.04"

FROM ubuntu:26.04@sha256:f144425ff09be612d6d9ad965196e9cdc23dae1f42110a8a11a3e9a8198759f7
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
