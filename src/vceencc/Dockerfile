# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/VCEEnc
ARG VCEENCC_VERSION="8.16"

ARG AMDGPU_VERSION="23.20.00.48"
ARG AMDGPU_INSTALL_VERSION="5.7.00.48.50700-1"
ARG DEPENDENCIES="initramfs-tools"

FROM public.ecr.aws/ubuntu/ubuntu:22.04
ARG VCEENCC_VERSION
ARG AMDGPU_VERSION
ARG AMDGPU_INSTALL_VERSION
ARG DEPENDENCIES

# hadolint ignore=DL3009
RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}
EOF

ADD https://github.com/rigaya/VCEEnc/releases/download/${VCEENCC_VERSION}/vceencc_${VCEENCC_VERSION}_Ubuntu22.04_amd64.deb /tmp/package.deb
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
