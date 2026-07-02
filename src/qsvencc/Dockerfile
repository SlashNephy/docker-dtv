# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG QSVENCC_VERSION="8.17"

FROM ubuntu:noble-20260509.1@sha256:786a8b558f7be160c6c8c4a54f9a57274f3b4fb1491cf65146521ae77ff1dc54
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
