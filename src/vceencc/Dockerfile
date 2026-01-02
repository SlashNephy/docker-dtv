# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/VCEEnc versioning=regex:^(?<major>\d+)\.(?<minor>\d+)$
ARG VCEENCC_VERSION="9.02"

# https://www.amd.com/en/support/download/linux-drivers.html
ARG AMDGPU_VERSION="23.20.00.48"
ARG AMDGPU_INSTALL_VERSION="5.7.00.48.50700-1"
ARG DEPENDENCIES="ca-certificates initramfs-tools"

# TODO: Ubuntu 24.04 (noble) ベースのイメージに更新する
FROM ubuntu:jammy-20251013@sha256:104ae83764a5119017b8e8d6218fa0832b09df65aae7d5a6de29a85d813da2fb
ARG VCEENCC_VERSION
ARG AMDGPU_VERSION
ARG AMDGPU_INSTALL_VERSION
ARG DEPENDENCIES

ADD https://github.com/rigaya/VCEEnc/releases/download/${VCEENCC_VERSION}/vceencc_${VCEENCC_VERSION}_Ubuntu22.04_amd64.deb /tmp/package.deb
# TODO: Ubuntu 24.04 (noble) 用のドライバに更新する
ADD https://repo.radeon.com/amdgpu-install/${AMDGPU_VERSION}/ubuntu/jammy/amdgpu-install_${AMDGPU_INSTALL_VERSION}_all.deb /tmp/amdgpu-install.deb

SHELL ["/usr/bin/bash", "-euxc"]
RUN <<EOF
  export DEBIAN_FRONTEND="noninteractive"
  export SUDO_FORCE_REMOVE="yes"

  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  apt-get install -y --no-install-recommends /tmp/amdgpu-install.deb
  amdgpu-install -y --accept-eula --usecase=graphics,amf,opencl --opencl=rocr,legacy --no-32

  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get purge -y --auto-remove ${DEPENDENCIES}
  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists
EOF

WORKDIR /
ENTRYPOINT [ "/usr/bin/vceencc" ]
