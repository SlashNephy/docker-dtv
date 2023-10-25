# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/VCEEnc
ARG VCEENCC_VERSION="8.16"

ARG AMDGPU_VERSION="23.10.3"
ARG AMDGPU_INSTALL_VERSION="5.5.50503-1"
ARG DEPENDENCIES="aria2 initramfs-tools"

FROM ubuntu:jammy@sha256:2b7412e6465c3c7fc5bb21d3e6f1917c167358449fecac8176c6e496e5c1f05f
ARG VCEENCC_VERSION
ARG AMDGPU_VERSION
ARG AMDGPU_INSTALL_VERSION
ARG DEPENDENCIES

ADD https://github.com/rigaya/VCEEnc/releases/download/${VCEENCC_VERSION}/vceencc_${VCEENCC_VERSION}_Ubuntu22.04_amd64.deb /tmp/package.deb

ENV SUDO_FORCE_REMOVE=yes
RUN <<EOF
  set -eux

  apt-get update

  cd /tmp
  apt-get install -y ${DEPENDENCIES}
  aria2c -x10 https://repo.radeon.com/amdgpu-install/${AMDGPU_VERSION}/ubuntu/jammy/amdgpu-install_${AMDGPU_INSTALL_VERSION}_all.deb --referer=www.amd.com
  apt-get install -y --no-install-recommends /tmp/amdgpu-install_${AMDGPU_INSTALL_VERSION}_all.deb
  amdgpu-install -y --accept-eula --usecase=graphics,amf,opencl --opencl=rocr,legacy --no-32

  apt-get update
  apt-get install -y --no-install-recommends /tmp/package.deb

  # clean up
  apt-get remove -y ${DEPENDENCIES}
  apt-get autoremove -y
  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists/*
EOF

ENTRYPOINT [ "/usr/bin/vceencc" ]
