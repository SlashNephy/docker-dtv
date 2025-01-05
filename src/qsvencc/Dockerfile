# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc
ARG QSVENCC_VERSION="7.76"

ARG DEPENDENCIES="curl ca-certificates gnupg2"

FROM public.ecr.aws/ubuntu/ubuntu:22.04@sha256:c6871ae8b54fb3ed0ba4df4eb98527e9a6692088fe0c2f2260a9334853092b47
ARG QSVENCC_VERSION
ARG DEPENDENCIES

ADD https://github.com/rigaya/QSVEnc/releases/download/${QSVENCC_VERSION}/qsvencc_${QSVENCC_VERSION}_Ubuntu22.04_amd64.deb /tmp/package.deb

RUN <<EOF
  set -eux

  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  # add Intel repository
  curl https://repositories.intel.com/gpu/intel-graphics.key | gpg --dearmor --output /usr/share/keyrings/intel-graphics.gpg
  echo "deb [arch=amd64 signed-by=/usr/share/keyrings/intel-graphics.gpg] https://repositories.intel.com/gpu/ubuntu jammy/production/2328 unified" | tee /etc/apt/sources.list.d/intel-gpu-jammy.list
  apt-get update

  apt-get install -y --no-install-recommends /tmp/package.deb

  apt-get remove -y ${DEPENDENCIES}
  apt-get autoremove -y
  apt-get clean
  rm -rf \
    /tmp/* \
    /var/lib/apt/lists/*
EOF

ENTRYPOINT [ "/usr/bin/qsvencc" ]
