# syntax=docker/dockerfile:1

ARG NODE_VERSION="20"

ARG DEPENDENCIES="curl gnupg"
ARG NVENCC_RUNTIMES="libcuda1-340 libswresample3 libavcodec58 libavformat58 libavfilter7"
ARG QSVENCC_RUNTIMES="intel-media-va-driver-non-free intel-opencl-icd intel-level-zero-gpu level-zero"
ARG RUNTIMES="nodejs ca-certificates ${NVENCC_RUNTIMES} ${QSVENCC_RUNTIMES}"

FROM ghcr.io/slashnephy/epgstation:20251231162821@sha256:2ffff01cf0b13490b9a0813cb1afb206e8bbf90fc0292fb4ace804c22b13b18c AS epgstation
FROM ghcr.io/slashnephy/nvencc:20251231163544@sha256:197e9ed7200ff91ad4f0eeee976cede81ef19f9ac4327c05bf2ccf27b674e6df AS nvencc
FROM ghcr.io/slashnephy/qsvencc:20251231164915@sha256:0a93400effef7d73d4e6f1a14e135b318d48b3cf29bb4b8902c63c9f862a334f AS qsvencc
FROM ghcr.io/slashnephy/vceencc:20251231175402@sha256:eaccea9f77bdf250bf33c4f301916c8adcd48cdf833ac9c059cf3e389ad4db63 AS vceencc

FROM ghcr.io/slashnephy/ffmpeg-ubuntu:20260101155658@sha256:9faad15dffdb215a549d8c2c122ca4951ba796fd107cbc8b13081f1161aa48d1
WORKDIR /app
ARG NODE_VERSION
ARG DEPENDENCIES
ARG RUNTIMES
ARG USERNAME="app"
ARG GROUPNAME="app"
ARG UID=1000
ARG GID=1000
ENV TZ="Asia/Tokyo"

RUN <<EOF
  set -eux

  groupadd -g "${GID}" "${GROUPNAME}"
  useradd -l -u "${UID}" -g "${GID}" "${USERNAME}"

  apt-get update
  apt-get install -y --no-install-recommends ${DEPENDENCIES}

  # Node
  curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /usr/share/keyrings/nodesource.gpg
  echo "deb [signed-by=/usr/share/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_VERSION}.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list

  # QSVEncC
  curl -fsSL https://repositories.intel.com/graphics/intel-graphics.key | gpg --dearmor -o /usr/share/keyrings/intel-graphics.gpg
  echo 'deb [arch=amd64,i386 signed-by=/usr/share/keyrings/intel-graphics.gpg] https://repositories.intel.com/graphics/ubuntu jammy arc' | tee  /etc/apt/sources.list.d/intel.gpu.jammy.list

  apt-get update
  apt-get install -y --no-install-recommends ${RUNTIMES}

  apt-get remove -y ${DEPENDENCIES}
  apt-get clean
  rm -rf /var/lib/apt/lists/*
EOF

COPY --from=epgstation --chown=$UID:$GID /app/ ./
COPY --from=epgstation --chown=$UID:$GID /app/client/ ./client/
COPY --from=nvencc /usr/bin/nvencc /usr/local/bin/
COPY --from=qsvencc /usr/bin/qsvencc /usr/local/bin/
COPY --from=vceencc /usr/bin/vceencc /usr/local/bin/

USER $USERNAME
ENTRYPOINT [ "node", "dist/index.js" ]
