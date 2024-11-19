# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc
ARG QSVENCC_VERSION="7.73"

ARG DEPENDENCIES="curl ca-certificates gnupg2"

FROM public.ecr.aws/ubuntu/ubuntu:24.04@sha256:da20fb875cfefd317c49e7aaf3998d3e5ad42c5b20f34a0eec6dca2fe4fbb8f4
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
