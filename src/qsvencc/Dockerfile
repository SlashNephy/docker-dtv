# syntax=docker/dockerfile:1

# renovate: datasource=github-releases depName=rigaya/QSVEnc
ARG QSVENCC_VERSION="7.94"

ARG DEPENDENCIES="curl ca-certificates gnupg2"

FROM public.ecr.aws/ubuntu/ubuntu:24.04@sha256:5dcf95d194f9781a99394c4084c4d2930fb7576c36f5abf817ccc13ef6a55c34
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
