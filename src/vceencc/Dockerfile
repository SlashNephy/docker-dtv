# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/VCEEnc
ARG VCEENCC_VERSION="9.02"

# https://www.amd.com/en/support/download/linux-drivers.html
ARG AMDGPU_VERSION="23.20.00.48"
ARG AMDGPU_INSTALL_VERSION="5.7.00.48.50700-1"
ARG DEPENDENCIES="ca-certificates initramfs-tools"

FROM ubuntu:noble-20251013@sha256:c35e29c9450151419d9448b0fd75374fec4fff364a27f176fb458d472dfc9e54
ARG VCEENCC_VERSION
ARG AMDGPU_VERSION
ARG AMDGPU_INSTALL_VERSION
ARG DEPENDENCIES
ENV DEBIAN_FRONTEND="noninteractive"

# hadolint ignore=DL3009
RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}
EOF

ADD https://github.com/rigaya/VCEEnc/releases/download/${VCEENCC_VERSION}/vceencc_${VCEENCC_VERSION}_Ubuntu24.04_amd64.deb /tmp/package.deb
# TODO: Ubuntu 24.04 (noble) 用のドライバに更新する
ADD https://repo.radeon.com/amdgpu-install/${AMDGPU_VERSION}/ubuntu/jammy/amdgpu-install_${AMDGPU_INSTALL_VERSION}_all.deb /tmp/amdgpu-install.deb

WORKDIR /tmp
ENV SUDO_FORCE_REMOVE=yes
RUN <<EOF
  set -eux

  apt-get install -y --no-install-recommends /tmp/amdgpu-install.deb
  amdgpu-install -y --accept-eula --usecase=graphics,amf,opencl --opencl=rocr,legacy --no-32

  apt-get update
  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get remove -y ${DEPENDENCIES}
  apt-get autoremove -y
  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists/*
EOF

WORKDIR /
ENTRYPOINT [ "/usr/bin/vceencc" ]
